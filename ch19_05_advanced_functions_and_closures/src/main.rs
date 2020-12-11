fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn function_pointers() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];

    let list_of_strings_closure: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_strings_named_fn: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    println!("closure: {:?}, fn: {:?}", list_of_strings_closure, list_of_strings_named_fn);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("tuple initializer: {:?}", list_of_statuses);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returning_closures() {
    let closure = returns_closure();
    println!("{:?}", closure(1));
}

fn main() {
    function_pointers();

    returning_closures();
}
