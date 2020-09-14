#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn area_basic(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    // area using decomposed parameters
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_basic(width1, height1)
    );

    // area using tuple dimensions
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // area using struct dimensions
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    // using derive debug annotation to print structs
    println!("rect1 is {:?}", rect1);
    // pretty print structs
    println!("rect1 is {:#?}", rect1);
}