use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn declarative_macros() {
    let v = myvec!(1, 2, 3);
    println!("myvec macro: {:?}", v);
}

#[derive(HelloMacro)]
struct Pancakes;

fn procedural_macros() {
    Pancakes::hello_macro();
}

fn main() {
    declarative_macros();

    // procedural macros (custom derive, attribute-like, and function-like)
    // MUST reside in own crate
    procedural_macros();
}
