fn matching_literals(x: i32) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables(x: Option<i32>, y: i32) {
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn matching_multiple_patterns(x: i32) {
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_numerical_range_patterns(x: i32) {
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn matching_char_range_patterns(x: char) {
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destruct_struct_named_variables(p: Point) {
    let Point { x: a, y: b } = p;
    assert_eq!(p.x, a);
    assert_eq!(p.y, b);
    println!("destruct named: {} {}", a, b);
}

fn destruct_struct_ordered_variables(p: Point) {
    let Point { x, y } = p;
    assert_eq!(p.x, x);
    assert_eq!(p.y, y);
    println!("destruct ordered: {} {}", x, y);
}

fn destruct_struct_with_literal_matching(p: Point) {
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destruct_enums(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}

enum NestedMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn destruct_nested_structs_and_enums(msg: NestedMessage) {
    match msg {
        NestedMessage::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        NestedMessage::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

fn main() {
    matching_literals(1); // useful for concrete values
    matching_named_variables(Some(5), 10); // be wary of shadowing mistakes w/scope
    matching_multiple_patterns(1); // | or operator for multi-pattern match
    matching_numerical_range_patterns(5); // simplifies | or when pattern is continuous range
    matching_char_range_patterns('c'); // chars can also multi-pattern match
    destruct_struct_named_variables(Point { x: 0, y: 7 }); // named variable destruct
    destruct_struct_ordered_variables(Point { x: 0, y: 7 }); // ordered variable destruct (shorthand)
    destruct_struct_with_literal_matching(Point { x: 0, y: 7 }); // struct matching patterns using destruct
    destruct_enums(Message::ChangeColor(0, 160, 255)); // enum destruct requires corresponding pattern
    destruct_nested_structs_and_enums(NestedMessage::ChangeColor(Color::Hsv(0, 160, 255))); // nested destruct patterns syntax sugar
}
