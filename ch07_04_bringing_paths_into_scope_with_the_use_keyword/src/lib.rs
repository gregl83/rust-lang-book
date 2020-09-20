use std::fmt;
use std::collections::*;
use std::time::{self, SystemTime};

mod front_of_house {
    pub mod hosting {
        use std::result::Result;
        use std::fmt;
        use std::io;

        pub fn add_to_waitlist() {
            println!("patron added to waitlist");
        }

        pub fn add_invoice() -> (fmt::Result, io::Result<()>) {
            (
                Result::Ok(()),
                Result::Ok(()),
            )
        }
    }
}

pub use self::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn reserve_seating_at_restaurant() {
    use self::front_of_house::hosting;
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn new_waitlist() -> HashMap<String,i32> {
    let waitlist = HashMap::new();

    println!("created new waitlist {:?}", waitlist);

    waitlist
}

pub fn clear_waitlist(waitlist: &mut HashMap<String,i32>) -> fmt::Result {
    use fmt::Result as Res;
    waitlist.clear();
    Res::Ok(())
}

pub fn add_invoice() {
    use self::front_of_house::hosting;
    let (a, b) = hosting::add_invoice();
    let a_val = match a {
        Ok(()) => 1,
        Err(error) => panic!("Error: {:?}", error),
    };
    let b_val = match b {
        Ok(()) => 1,
        Err(error) => panic!("Error: {:?}", error),
    };
    println!("Invoice Results ({}, {})", a_val, b_val);
}

pub fn create_reservation_id() -> i32 {
    use rand::Rng;
    rand::thread_rng().gen_range(1, 1000)
}

pub fn get_now() -> (time::Instant, SystemTime) {
    (time::Instant::now(), SystemTime::now())
}