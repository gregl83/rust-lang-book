fn main() {
    // vector: initialize with value type i32
    let vector_explicit: Vec<i32> = Vec::new();

    println!("{:?}", vector_explicit);

    // vector: initialize with values of 1, 2, and 3 inferred using macro
    let vector_inferred = vec![1, 2, 3];

    println!("{:?}", vector_inferred);

    // vector: updating
    let mut vector_updated = vec![1, 2, 3];
    vector_updated.push(4);
    vector_updated.push(5);
    vector_updated.push(6);

    println!("{:?}", vector_updated);

    // vector: out-of-scope values are cleaned up
    {
        let vector_scope = vec![1];

        println!("{:?} cleaned after scope", vector_scope);
    }

    // vector: referencing values
    let vector_reference = vec![1, 2, 3, 4, 5];

    let third: &i32 = &vector_reference[2];
    println!("The third element is {}", third);

    match vector_reference.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // vector: iterate values
    let vector_iterate = vec![100, 32, 57];
    for i in &vector_iterate {
        println!("{}", i);
    }

    let mut vector_mut_iterate = vec![100, 32, 57];
    for i in &mut vector_mut_iterate {
        *i += 50;
    }
    println!("{:?}", vector_mut_iterate);

    // vector: multiple value types using enums
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
