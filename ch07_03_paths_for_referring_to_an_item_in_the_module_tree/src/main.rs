use ch07_03_paths_for_referring_to_an_item_in_the_module_tree::{
    eat_at_restaurant,
    settle_invoice,
    order_at_restaurant,
    review_restaurant,
};

fn main() {
    // call module function
    eat_at_restaurant();

    // call module super
    settle_invoice();

    // struct scoping (default private require public)
    order_at_restaurant();

    // enum scoping
    review_restaurant(5);
}