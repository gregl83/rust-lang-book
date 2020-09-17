mod front_of_house {
    #[derive(Debug)]
    pub enum Review {
        OneStar,
        TwoStars,
        ThreeStars,
        FourStars,
        FiveStars,
    }

    pub mod hosting {
        use std::time::SystemTime;

        pub fn add_to_waitlist() -> SystemTime {
            SystemTime::now()
        }

        fn _seat_at_table() {}
    }

    pub mod serving {
        use std::time::SystemTime;

        fn _take_order() {}

        fn _serve_order() {}

        pub fn take_payment() -> SystemTime {
            super::exit();
            SystemTime::now()
        }
    }

    fn exit() {
        println!("exit called");
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    println!(
        "absolute module path timestamp: {:?}",
        crate::front_of_house::hosting::add_to_waitlist()
    );

    // relative path
    println!(
        "relative module path timestamp: {:?}",
        front_of_house::hosting::add_to_waitlist()
    );
}

pub fn order_at_restaurant() {
    // order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    // change our mind about what bread we'd like
    meal.toast = String::from("wheat");

    println!("i'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); - cannot use due to scope privacy
}

pub fn settle_invoice() {
    // absolute path
    println!(
        "parent/super call timestamp: {:?}",
        crate::front_of_house::serving::take_payment()
    );
}

pub fn review_restaurant(stars: i8) {
    let review = match stars {
        0 | 1 => front_of_house::Review::OneStar,
        2 => front_of_house::Review::TwoStars,
        3 => front_of_house::Review::ThreeStars,
        4 => front_of_house::Review::FourStars,
        _ => front_of_house::Review::FiveStars,
    };

    println!("review restaurant: {:?}", review);
}