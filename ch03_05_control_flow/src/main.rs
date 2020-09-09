fn fahrenheit_to_celsius(degrees: f32) -> f32 {
    (degrees - 32.0) * (5.0 / 9.0)

}

fn celsius_to_fahrenheit(degrees: f32) -> f32 {
    degrees * (9.0 / 5.0) + 32.0
}

fn fibonacci(number: u32) -> u32 {
    if number == 0 || number == 1 {
        return number;
    }
    fibonacci(number - 1) + fibonacci(number - 2)
}

const ORDINAL_LABELS: [(&str, &str); 12] = [
    ("One", "first"),
    ("Two", "second"),
    ("Three", "third"),
    ("Four", "fourth"),
    ("Five", "fifth"),
    ("Six", "sixth"),
    ("Seven", "seventh"),
    ("Eight", "eigth"),
    ("Nine", "nineth"),
    ("Ten", "tenth"),
    ("Eleven", "eleventh"),
    ("Twelve", "twelth"),
];

fn print_days_of_christmas(items: [&str; 12]) {
    let mut ordinal_labels = &mut ORDINAL_LABELS;
    ordinal_labels.reverse();
    let mut items= items;
    items.reverse();
    let last_item_index = items.len() - 1;

    for verse_number in 0..items.len() {
        println!("On the {} day of Christmas", ORDINAL_LABELS[verse_number].1);
        println!("My true love gave to me");

        let verse_start_item = items.len() - verse_number - 1;
        let verse_end_item = items.len();
        let verse_items = &items[verse_start_item..verse_end_item];

        for (item_index, item) in verse_items.iter().enumerate() {
            let mut prefix = ordinal_labels[verse_start_item + item_index].0;

            if item_index == verse_items.len() - 1 {
                prefix = if verse_number == 0 { "A" } else { "And a" };
            }

            println!("{} {}", prefix, item);
        }

        if verse_number < items.len() - 1 {
            println!("");
        }
    }
}

fn main() {
    // expression: if and else
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // expression: if requires boolean
    let zero = 0;
    if zero == 0 {
        println!("condition was true");
    }

    // expression: else if
    let two = 2;
    if two == 1 {
        println!("condition met with numeric 1");
    } else if two == 2 {
        println!("condition met with numeric 2");
    } else {
        println!("condition not met");
    }

    // expression: ternary
    let ternary_boolean = false;
    let ternary_returned = if ternary_boolean { 1 } else { 0 };

    println!("condition was {}", ternary_returned);

    // loop: with conditional exit
    let mut count = 0;
    let count_break = 5;
    loop {
        count += 1;
        println!("loop count was {}", count);
        if count == count_break { break };
    }

    // loop: with return values
    let mut count_return = 0;
    let loop_result = loop {
        count_return += 1;

        if count_return == 10 {
            break count_return * 2;
        }
    };

    println!("The loop_result is {}", loop_result);

    // while: countdown
    let mut countdown = 3;
    while countdown != 0 {
        println!("{}!", countdown);
        countdown -= 1;
    }

    println!("LIFTOFF!!!");

    // while: iterate array
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", array[index]);
        index += 1;
    }

    // for: iterate array
    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("the value is: {}", element);
    }

    // for: countdown
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // example: temperature conversion
    let control = 0.0;
    let degrees_fahrenheit = celsius_to_fahrenheit(control);
    let degrees_celsius = fahrenheit_to_celsius(degrees_fahrenheit);
    println!("degree conversions {}F and {}C", degrees_fahrenheit, degrees_celsius);

    // example: fibonacci
    let fib_zero = fibonacci(0);
    let fib_one = fibonacci(1);
    let fib_seven = fibonacci(7);
    println!("fib_zero: {} fib_one: {} fib_seven: {}", fib_zero, fib_one, fib_seven);

    // example: days of christmas
    let items: [&str; 12] = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "gold rings, badam-pam-pam",
        "geese a laying",
        "swans a swimming",
        "maids a milking",
        "ladies dancing",
        "lords a leaping",
        "pipers piping",
        "drummers drumming",
    ];
    print_days_of_christmas(items);
}
