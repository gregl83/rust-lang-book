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
    // custom derive
    Pancakes::hello_macro();


    // attribute-like

    // #[route(GET, "/")]
    // fn index() {
    //
    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {


    // function-like macros

    // let sql = sql!(SELECT * FROM posts WHERE id=1);
    //
    // #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {
}

fn main() {
    declarative_macros();

    // procedural macros (custom derive, attribute-like, and function-like)
    // MUST reside in own crate
    procedural_macros();
}
