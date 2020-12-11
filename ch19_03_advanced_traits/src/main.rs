use std::ops::Add;

trait Iterator {
    type Item;

    fn next(&mut  self) -> Option<Self::Item>;
}

#[derive(Debug)]
struct Counter;

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // fixme - this doesn't iterate /shrug
        Some(1)
    }
}

fn trait_types() {
    let counter = Counter {};

    println!("counter: {:?}", counter);
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn default_type_overload_overwrite() {
    let mm = Millimeters(1);
    let m = Meters(1);

    let result = mm + m;

    println!("mm + m = {:?}", result);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// ----

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn fully_qualified_syntax() {
    let person = Human;

    Pilot::fly(&person);

    Wizard::fly(&person);

    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // fully qualified:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
}

fn main() {
    trait_types();

    default_type_overload_overwrite();

    fully_qualified_syntax();
}
