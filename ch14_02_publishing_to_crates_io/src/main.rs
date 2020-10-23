use ch14_02_publishing_to_crates_io::{PrimaryColor, mix, one};

fn main() {
    let unsigned_int = one();

    println!("documented function: {}", unsigned_int);

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let mixed = mix(red, yellow);

    println!("{:?}", mixed);
}
