mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天訂購一個黑麥土司作為早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改變主意更換想要麵包的型別
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的註釋程式碼不能編譯；
    // 不允許檢視或修改早餐附帶的季節水果
    // meal.seasonal_fruit = String::from("blueberries");
}
