fn first_word_location(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    // slice of mutable string
    let mut mutable_string = String::from("mutable string");
    let first_word_found = first_word_location(&mutable_string);

    println!("First word of \"{}\": {}", mutable_string, first_word_found);

    mutable_string.clear();

    println!("First word of \"{}\": {}", mutable_string, first_word_found);

    // string slice: immutable string
    let immutable_string = String::from("unmutable string");
    let immutable = &immutable_string[0..9];
    let string = &immutable_string[10..16];

    println!("Slices \"{}\" and \"{}\" from: \"{}\"", immutable, string, immutable_string);

    // string slices: start and end range
    let string_range = String::from("string range");
    let string_start_wildcard = &string_range[..string_range.len()];
    let string_end_wildcard = &string_range[0..];

    println!("Start {} and end {}", string_start_wildcard, string_end_wildcard);

    // string slices: first word slice
    let string_slices_first_word = String::from("string slices first word");
    let slice_first_word_found = first_word_slice(&string_slices_first_word);

    println!("First word of slice \"{}\": {}", string_slices_first_word, slice_first_word_found);

    // string literal: binary slice
    let string_literal = "binary slice";

    println!("String literal binary slice: {}", string_literal);

    // array slice
    let array = [1, 2, 3];
    let array_slice = &array[0..1];

    println!("Array {:?} slice {:?}", array, array_slice);
}
