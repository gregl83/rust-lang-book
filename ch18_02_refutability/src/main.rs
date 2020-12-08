fn refutable_pattern(some_option_value: Option<i32>) {
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}

fn irrefutable_pattern(some_option_value: Option<i32>) {
    match some_option_value {
        Some(x) => println!("{}", x),
        None => println!("{}", "none")
    }

}

fn main() {
    refutable_pattern(Some(1));
    irrefutable_pattern(None);
}
