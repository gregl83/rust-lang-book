use String;

fn takes_ownership(some_string: String) {
    println!("Takes ownership: {}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn makes_copy(some_integer: i32) {
    println!("Makes copy: {}", some_integer);
}

fn takes_and_gives_back_with_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    // string literal scope
    {
        let s: &str = "string literal";

        println!("String literal in-scope: {}", s)
    }

    // string runtime heap
    let mut string_heap = String::from("string");
    string_heap.push_str(" heap");

    println!("String in-heap: {}", string_heap);

    // string move new variable
    let string_move = String::from("string move");
    let string_moved = string_move;

    println!("String move: {}", string_moved);

    // string copy
    let string_copy = String::from("string copy");
    let string_copied = string_copy.clone();

    println!("String copy: {} String copy clone: {}", string_copy, string_copied);

    // integer copy (primitive)
    let integer_copy = 13;
    let integer_copied = integer_copy;

    println!("Integer copy: {} Integer copied: {}", integer_copy, integer_copied);

    // function parameters move and copy
    let function_move = String::from("function move");
    takes_ownership(function_move);
    let function_copy = 5;
    makes_copy(function_copy);

    // function return values
    let function_receive = gives_ownership();
    println!("Function receive: {}", function_receive);
    let function_give = String::from("function give");
    println!("Function give: {}", function_give);
    let function_take_back = takes_and_gives_back(function_give);
    println!("Function take back: {}", function_take_back);

    // function return multiple values
    let function_length_give = String::from("function length give");
    let (function_length_take_back, function_length) = takes_and_gives_back_with_length(function_length_give);
    println!("The length of '{}' is {}.", function_length_take_back, function_length);
}
