use std::collections::HashMap;
mod math;

fn main() {
    // hashmap new w/inserting
    let mut scores = HashMap::new();

    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 10);

    println!("{:?}", scores);

    // hashmap from iterators
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);

    // hashmap with owned values
    let owned_name = String::from("name");
    let owned_value = String::from("value");

    let mut map = HashMap::new();
    map.insert(owned_name, owned_value);

    println!("now owned by map: {:?}", map);

    // hashmap access values
    let mut latest_scores = HashMap::new();

    latest_scores.insert(String::from("Blue"), 10);
    latest_scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = latest_scores.get(&team_name);

    println!("team: {} score: {:?}", team_name, score);

    // hashmap iteration
    let mut scores_map = HashMap::new();

    scores_map.insert(String::from("Blue"), 10);
    scores_map.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // hashmap overwrite with new value
    let mut scores_overwrite = HashMap::new();

    scores_overwrite.insert(String::from("Blue"), 10);
    scores_overwrite.insert(String::from("Blue"), 25);

    println!("{:?}", scores_overwrite);

    // hashmap insert if key not exists
    let mut scores_conditional_insert = HashMap::new();

    scores_conditional_insert.insert(String::from("Blue"), 10);

    scores_conditional_insert.entry(String::from("Yellow")).or_insert(50);
    scores_conditional_insert.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores_conditional_insert);

    // hashmap update value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let a = 7 % 2;
    println!("{}", a);

    let vector = vec![1.0, 2.0, 3.0];
    println!("mean: {:?}", math::vectors::mean(vector));

    let vector = vec![1.0, 2.0, 3.0, 4.0];
    println!("median: {:?}", math::vectors::median(vector));

    let vector = vec![1, 2, 3, 4, 2, 3];
    println!("mode: {:?}", math::vectors::mode(vector));
}
