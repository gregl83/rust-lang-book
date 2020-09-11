fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" reference string");
}

fn no_dangle() -> String {
    let s = String::from("no dangle");
    s
}

fn main() {
    // function parameter by reference
    let reference_string = String::from("reference string");
    let reference_length = calculate_length(&reference_string);

    println!("The length of '{}' is {}.", reference_string, reference_length);

    // function parameter mutable by reference
    let mut reference_string_mutable = String::from("mutated");
    change(&mut reference_string_mutable);

    println!("The mutated reference string is now {}", reference_string_mutable);

    // mutable multiple scoping references
    let mut mutable_scope_reference = String::from("mutable scope reference");
    {
        let mutable_reference = &mut mutable_scope_reference;
    }
    let mutable_reference_scoped = &mut mutable_scope_reference;

    println!("Value for mutable reference scoped: {}", mutable_reference_scoped);

    // mutable multiple references
    let mut multiple_reference = String::from("mutable reference");
    let r1 = &multiple_reference;
    let r2 = &multiple_reference;

    println!("Value for mutable references: {} and {}", r1, r2);

    // dangling references
    let undangled = no_dangle();

    println!("Value for no-dangle (uses move): {}", undangled);
}