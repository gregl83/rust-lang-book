use std::fmt;
use std::io::Error;

type Kilometers = i32;

// type aliases for brevity

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    f();
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("returned"))
}

// result type alias w/generic:
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn creating_type_aliases() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Thunk = returns_long_type();
    takes_long_type(f);
}

fn never_type_returns_never() -> ! {
    panic!("never return")
}

fn generic<T: ?Sized + fmt::Display>(t: &T) {
    println!("{}", t);
}

fn dynamically_sized_types() {
    generic("hello generic dst!");
}

fn main() {
    creating_type_aliases();

    // never_type_returns_never();

    dynamically_sized_types(); // DST
}
