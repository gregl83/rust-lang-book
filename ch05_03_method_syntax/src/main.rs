#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // struct implement method
    let rect_method = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_method.area()
    );

    // struct method multiple params
    let rect_multiple1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect_multiple2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect_multiple3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect_multiple1 hold rect_multiple2? {}", rect_multiple1.can_hold(&rect_multiple2));
    println!("Can rect_multiple1 hold rect_multiple3? {}", rect_multiple1.can_hold(&rect_multiple3));

    // struct associated function
    let square = Rectangle::square(50);

    println!("{:#?}", square);

    // struct class method
    let rect1_class = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        Rectangle::area(&rect1_class)
    );

}
