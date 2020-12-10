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

fn main() {
    trait_types();
}
