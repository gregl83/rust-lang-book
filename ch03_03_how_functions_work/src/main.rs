fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn return_implicit() -> u16 {
    24 * 7
}

fn return_explicit(integer: u16) -> u16 {
    return integer + 1;
}

fn main() {
    println!("Hello, world!");

    // calling another function
    another_function(5, 6);

    // statements
    let alpha = 1;

    // expression inside statement
    let bravo = (alpha + 1);

    // expression block
    let charlie = {
        bravo + 1
    };

    // return value implicitly
    let returned_implicitly = return_implicit();

    println!("The value of returned_implicit is: {}", returned_implicitly);

    let returned_explicitly = return_explicit(returned_implicitly);

    println!("The value of returned_explicitly is: {}", returned_explicitly);
}