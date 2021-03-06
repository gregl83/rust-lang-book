use ch13_01_closures::generate_workout;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    {
        let x = 4;

        let equal_to_x = |z| z == x;

        let y = 4;

        assert!(equal_to_x(y));
    }
}
