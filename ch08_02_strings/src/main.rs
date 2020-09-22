fn main() {
    // new string
    let new_string = String::new();
    println!("new string: {:?}", new_string);

    // string to_string display trait method
    let data = "initial contents";
    println!("{}", data);
    let string_from_str = data.to_string();
    println!("{}", string_from_str);
    let string_from_str = "initial contents".to_string();
    println!("{}", string_from_str);

    // string encoding is utf-8
    println!("{}", String::from("السلام عليكم"));
    println!("{}", String::from("Dobrý den"));
    println!("{}", String::from("Hello"));
    println!("{}", String::from("שָׁלוֹם"));
    println!("{}", String::from("नमस्ते"));
    println!("{}", String::from("こんにちは"));
    println!("{}", String::from("안녕하세요"));
    println!("{}", String::from("你好"));
    println!("{}", String::from("Olá"));
    println!("{}", String::from("Здравствуйте"));
    println!("{}", String::from("Hola"));

    // string append
    let mut string_append = String::from("foo");
    string_append.push_str("bar");
    println!("{}", string_append);

    // string append using slice
    let mut string_append_slice = String::from("foo");
    let string_slice = "bar";
    string_append_slice.push_str(string_slice);
    println!("s2 is {}", string_slice);

    // string push single character
    let mut single_character = String::from("lo");
    single_character.push('l');
    println!("{}", single_character);

    // string concat using + operator
    let string_hello = String::from("Hello, ");
    let string_world = String::from("world!");
    let string_hello_world = string_hello + &string_world;
    // note string_hello has been moved above, not borrowed, and can no longer be used
    println!("{}", string_world);
    println!("{}", string_hello_world);

    // string concatenation with more than two values
    let string_one_basic = String::from("tic");
    let string_one_formatted = string_one_basic.clone();
    let string_two = String::from("tac");
    let string_three = String::from("toe");

    let basic_concat = string_one_basic + "-" + &string_two + "-" + &string_three;
    println!("{}", basic_concat);

    let formatted_concat = format!("{}-{}-{}", string_one_formatted, string_two, string_three);
    println!("{}", formatted_concat);

    // string slice (bytes vs grapheme cluster)
    let russian_hello = "Здравствуйте";
    let russian_hello_slice = &russian_hello[0..4];
    println!("{} is [0..4] of {}", russian_hello_slice, russian_hello);

    // string iteration: unicode scalar values
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // string iteration: bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
