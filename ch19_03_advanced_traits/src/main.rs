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

fn main() {
    trait_types();

    default_type_overload_overwrite();
}
