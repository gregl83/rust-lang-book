#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &Message {
        self
    }
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}

fn main() {
    // enum variants of ip address (either four or six not both)
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // enum any variant as function argument
    println!(
        "IP Address Kind: {:?}",
        route(IpAddrKind::V4)
    );
    println!(
        "IP Address Kind: {:?}",
        route(IpAddrKind::V6)
    );

    // enum with value types
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    // enum variants with method
    let m = Message::Write(String::from("hello"));
    println!(
        "Message method returns self: {:?}",
        m.call()
    );

    // enum option
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!(
        "some number: {:?} some string: {:?} absent number: {:?}",
        some_number,
        some_string,
        absent_number
    );
}
