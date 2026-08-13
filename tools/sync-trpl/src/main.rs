use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const README_HEADER: &str = "\
> 本專案為 [trpl-zh-cn](https://github.com/KaiserY/trpl-zh-cn) 的繁體中文自動化翻譯版本,
> 原文版權歸屬 Rust 中文社區(MIT License)。翻譯內容由 OpenCC 自動轉換產生,可能有未校對之處。

---

";
// 這些字串在套用術語詞典時完全跳過,不會被任何規則替換
const PROTECTED_STRINGS: &[&str] = &["簡體中文版.pdf", "簡體中文譯本", "簡體中文："];

const UPSTREAM: &str = "upstream/main";
const LAST_SYNCED_FILE: &str = "tools/.last_synced_commit";
const CONVERT_CONFIG: &str = "s2twp"; // Simplified -> Traditional (Taiwan)
const TERMS_DICT_FILE: &str = "tools/terms.dict";

// 只有這些副檔名會經過 OpenCC + 術語詞典翻譯,其他檔案原封不動複製
const TRANSLATABLE_EXTENSIONS: &[&str] = &["md", "toml", "json", "rs"];

// 這些路徑前綴完全略過(不複製、不刪除、不翻譯),避免蓋掉你自己的基礎設施
const EXCLUDE_PREFIXES: &[&str] = &[".github/"];

#[derive(Debug)]
enum Status {
    Added,
    Modified,
    Deleted,
}

fn run_git(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").args(args).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains("unknown revision") && !stderr.contains("does not exist") {
            eprintln!("git {} 執行失敗:\n{}", args.join(" "), stderr);
        }
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_last_synced() -> Option<String> {
    fs::read_to_string(LAST_SYNCED_FILE)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn get_current_upstream_commit() -> Result<String, Box<dyn Error>> {
    run_git(&["rev-parse", UPSTREAM])
}

fn is_translatable(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|ext| TRANSLATABLE_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
}

fn is_excluded(path: &str) -> bool {
    EXCLUDE_PREFIXES.iter().any(|p| path.starts_with(p))
}

/// 回傳 (Status, path)。不限制 pathspec,抓整個 repo 的變動。
fn get_changed_files(
    last_synced: &Option<String>,
    current: &str,
) -> Result<Vec<(Status, String)>, Box<dyn Error>> {
    let mut changes = Vec::new();

    match last_synced {
        None => {
            // 第一次同步,抓 upstream 目前所有檔案當作新增
            let output = run_git(&["ls-tree", "-r", "--name-only", current])?;
            for path in output.lines().filter(|l| !l.is_empty()) {
                changes.push((Status::Added, path.to_string()));
            }
        }
        Some(last) => {
            let output = run_git(&["diff", "--name-status", last.as_str(), current])?;

            for line in output.lines().filter(|l| !l.is_empty()) {
                let parts: Vec<&str> = line.split('\t').collect();
                let status_code = parts[0];

                if status_code.starts_with('R') {
                    // rename: status\told_path\tnew_path
                    if parts.len() >= 3 {
                        changes.push((Status::Deleted, parts[1].to_string()));
                        changes.push((Status::Added, parts[2].to_string()));
                    }
                } else if parts.len() >= 2 {
                    let path = parts[1].to_string();
                    match status_code {
                        "A" => changes.push((Status::Added, path)),
                        "M" => changes.push((Status::Modified, path)),
                        "D" => changes.push((Status::Deleted, path)),
                        other => eprintln!("未處理的狀態 {}: {}", other, path),
                    }
                }
            }
        }
    }

    Ok(changes
        .into_iter()
        .filter(|(_, p)| !is_excluded(p))
        .collect())
}

/// 以文字方式取得檔案內容(給 .md/.toml 這類翻譯用)
fn get_upstream_file_content(path: &str) -> Result<Option<String>, Box<dyn Error>> {
    let spec = format!("{}:{}", UPSTREAM, path);
    let output = Command::new("git").args(["show", &spec]).output()?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
}

/// 以原始 bytes 取得檔案內容(給圖片等二進位檔案用,避免 UTF-8 轉換破壞內容)
fn get_upstream_file_bytes(path: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    let spec = format!("{}:{}", UPSTREAM, path);
    let output = Command::new("git").args(["show", &spec]).output()?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(Some(output.stdout))
}

fn convert_with_opencc(content: &str) -> Result<String, Box<dyn Error>> {
    let mut child = Command::new("opencc")
        .args(["-c", &format!("{}.json", CONVERT_CONFIG)])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or("無法取得 opencc stdin")?
        .write_all(content.as_bytes())?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err("opencc 轉換失敗".into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// 載入自訂術語詞典(TSV: 來源詞\t目標詞)
/// 長詞優先比對,避免短詞子字串誤替換長詞
fn load_terms_dict(path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path)?;
    let mut terms: Vec<(String, String)> = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .filter_map(|line| {
            let mut parts = line.splitn(2, '\t');
            let from = parts.next()?.trim();
            let to = parts.next()?.trim();
            if from.is_empty() || to.is_empty() {
                None
            } else {
                Some((from.to_string(), to.to_string()))
            }
        })
        .collect();

    terms.sort_by(|a, b| b.0.chars().count().cmp(&a.0.chars().count()));

    Ok(terms)
}

fn apply_terms_dict(content: &str, terms: &[(String, String)]) -> String {
    if terms.is_empty() {
        return content.to_string();
    }
    // Step 1: 把保護字串換成不會跟原文衝突的佔位符
    let mut working = content.to_string();
    let mut placeholders: Vec<(String, &str)> = Vec::new();

    for (i, protected) in PROTECTED_STRINGS.iter().enumerate() {
        if working.contains(protected) {
            let placeholder = format!("\u{E000}PROTECTED_{}\u{E000}", i); // 用 Unicode 私用區字元當標記,幾乎不可能跟原文衝突
            working = working.replace(protected, &placeholder);
            placeholders.push((placeholder, protected));
        }
    }

    // Step 2: 正常套用術語詞典
    for (from, to) in terms {
        working = working.replace(from.as_str(), to.as_str());
    }

    // Step 3: 把佔位符換回原本的保護字串
    for (placeholder, original) in placeholders {
        working = working.replace(&placeholder, original);
    }

    working
}

/// 依副檔名決定處理方式:.md/.toml 翻譯,其他檔案原樣複製(二進位安全)
fn convert_and_write(path: &str, terms: &[(String, String)]) -> Result<bool, Box<dyn Error>> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    if is_translatable(path) {
        let content = match get_upstream_file_content(path)? {
            Some(c) => c,
            None => {
                eprintln!("警告：無法取得 {} 的內容，略過", path);
                return Ok(false);
            }
        };
        let converted = convert_with_opencc(&content)?;
        let mut final_content = apply_terms_dict(&converted, terms);

        // README.md 額外加上固定的來源標註區塊
        if path == "README.md" {
            final_content = format!("{}{}", README_HEADER, final_content);
        }

        fs::write(path, final_content)?;
    } else {
        let bytes = match get_upstream_file_bytes(path)? {
            Some(b) => b,
            None => {
                eprintln!("警告：無法取得 {} 的內容，略過", path);
                return Ok(false);
            }
        };
        fs::write(path, bytes)?;
    }

    Ok(true)
}

fn delete_file(path: &str) {
    if Path::new(path).exists() {
        match fs::remove_file(path) {
            Ok(_) => println!("已刪除: {}", path),
            Err(e) => eprintln!("刪除 {} 失敗: {}", path, e),
        }
    } else {
        println!("檔案本來就不存在，略過刪除: {}", path);
    }
}

/// 全量重跑:不管 upstream 有沒有變動,把目前 upstream 上
/// 所有檔案重新處理一次(翻譯檔重轉、其他檔重複製)。用在詞典更新後的重刷。
fn rebuild_all(terms: &[(String, String)]) -> Result<(), Box<dyn Error>> {
    let current = get_current_upstream_commit()?;

    let output = run_git(&["ls-tree", "-r", "--name-only", &current])?;
    let files: Vec<&str> = output
        .lines()
        .filter(|l| !l.is_empty() && !is_excluded(l))
        .collect();

    println!("重新處理全部 {} 個檔案", files.len());

    for path in files {
        match convert_and_write(path, terms) {
            Ok(true) => println!("已重新處理: {}", path),
            Ok(false) => {}
            Err(e) => eprintln!("處理 {} 時發生錯誤: {}", path, e),
        }
    }

    fs::write(LAST_SYNCED_FILE, &current)?;
    println!("同步紀錄已更新為: {}", current);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let force_rebuild = args.iter().any(|a| a == "--rebuild-all");

    run_git(&["fetch", "upstream"])?;

    let terms = load_terms_dict(TERMS_DICT_FILE)?;
    println!("已載入 {} 條自訂術語規則", terms.len());

    if force_rebuild {
        return rebuild_all(&terms);
    }

    let last_synced = get_last_synced();
    let current = get_current_upstream_commit()?;

    if last_synced.as_deref() == Some(current.as_str()) {
        println!("沒有新的變動，跳過同步。");
        return Ok(());
    }

    let changes = get_changed_files(&last_synced, &current)?;

    if changes.is_empty() {
        println!("沒有找到變動檔案。");
    } else {
        println!("共 {} 項變動：", changes.len());
        for (status, path) in &changes {
            println!("  [{:?}] {}", status, path);
        }

        for (status, path) in &changes {
            match status {
                Status::Deleted => delete_file(path),
                Status::Added | Status::Modified => match convert_and_write(path, &terms) {
                    Ok(true) => println!("已處理: {}", path),
                    Ok(false) => {}
                    Err(e) => eprintln!("處理 {} 時發生錯誤: {}", path, e),
                },
            }
        }
    }

    fs::write(LAST_SYNCED_FILE, &current)?;
    println!("同步紀錄已更新為: {}", current);

    Ok(())
}
