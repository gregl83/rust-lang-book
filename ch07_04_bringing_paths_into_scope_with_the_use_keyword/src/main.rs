use ch07_04_bringing_paths_into_scope_with_the_use_keyword::{
    eat_at_restaurant,
    reserve_seating_at_restaurant,
    new_waitlist,
    clear_waitlist,
    add_invoice,
    add_to_waitlist,
    create_reservation_id,
    get_now,
};

fn main() {
    // use statement with absolute path
    eat_at_restaurant();

    // use statement with relative path
    reserve_seating_at_restaurant();

    // use statement idiomatic paths
    let waitlist = &mut new_waitlist();

    // use statement idiomatic same name
    add_invoice();

    // use alias supporting collision avoidance
    let _ = clear_waitlist(waitlist);

    // use re-exporting for public
    add_to_waitlist();

    // use external packages
    println!(
        "reservation id: {}",
        create_reservation_id()
    );

    // use self and property
    let (instant, system_time) = get_now();
    println!(
        "{:?} {:?}",
        instant,
        system_time,
    )
}