use ch10_02_traits::{
    Tweet,
    NewsArticle,
    Summary,
    Pair,
    notify,
    top_two,
    returns_summarizable
};

#[derive(Debug)]
enum OptionF64 {
    Some(f64),
    None,
}

fn largest<T>(list: &[T]) -> &T where T: PartialOrd + Copy {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet);
    notify(&article);

    top_two(&tweet, &article);

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));

    let ints = vec![8, 5, 2, 7];
    println!("largest int: {}", largest(&ints));

    let pair = Pair::new(3, 2);
    pair.cmp_display();

    // blanket implementation: "implement a trait for any type that implements another trait"
    // for example: <T: Display> ToString for T { ... }
}
