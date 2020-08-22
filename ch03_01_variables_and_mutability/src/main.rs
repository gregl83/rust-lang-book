// constant in global scope
const OFFSET: u8 = 0;

fn main() {
    // variables (immutable)
    let x = 5;
    println!("The value of x is: {}", x);
    // -- commented out to prevent compiler error --
    // x = 6;
    // println!("The value of x is: {}", x);

    // variables (mutable)
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // constants
    const YEAR: u16 = 2020;
    println!("The year {} with offset {}", YEAR, OFFSET);

    // shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}