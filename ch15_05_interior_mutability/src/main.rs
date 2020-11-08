fn main() {
    let mut a = 1;
    let mut b = &a;
    let mut c = &a;

    a = 3;

    println!("{}, {}, {}", a, b, c);

    println!("Hello, world!");
}
