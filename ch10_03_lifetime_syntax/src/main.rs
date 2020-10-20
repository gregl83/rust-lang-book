use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn return_string<'a>() -> &'a str {
    "string"
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");

    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    // let reference: i32;
    // let reference_lifetime: &'a i32;
    // let reference_mut_lifetime: &'a mut i32;

    println!("return {} static literal", return_string());

    let i;
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    println!("level: {}", i.level());
    println!("announce and return: {}", i.announce_and_return_part("announce!"));

    println!("{}", longest_with_an_announcement("one", "three", "announce!"));
}
