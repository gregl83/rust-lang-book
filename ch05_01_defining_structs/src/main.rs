#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // struct: new user struct
    let user_immutable = User {
        username: String::from("gregl83"),
        email: String::from("admin@gregorylanglais.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("{:?}", user_immutable);

    // struct: mutable new user struct
    let mut user_mutable = User {
        username: String::from("gregl83"),
        email: String::from("general@gregorylanglais.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("{:?}", user_mutable);

    user_mutable.email = String::from("admin@gregorylanglais.com");

    println!("{:?}", user_mutable);

    // struct: build function with field init shorthand
    let user_built = build_user(
        String::from("admin@gregorylanglais.com"),
        String::from("gregl83"),
    );

    println!("{:?}", user_built);

    // struct: update syntax
    let user_original = build_user(
        String::from("admin@gregorylanglais.com"),
        String::from("gregl83"),
    );

    let user_additional = User {
        email: String::from("general@gregorylanglais.com"),
        username: String::from(&user_original.username),
        ..user_original
    };

    println!("User original: {:?}", user_original);
    println!("User additional: {:?}", user_additional);

    // structs: tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Tuple struct black: ({}, {}, {})", black.0, black.1, black.2);
    println!("Tuple struct origin: ({}, {}, {})", origin.0, origin.1, origin.2);
}
