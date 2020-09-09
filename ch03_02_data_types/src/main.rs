fn main() {
    // string to int type annotation
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess is: {}", guess);

    // scalar: integer
    let _uint8: u8 = 127;
    let _int8: i8 = -128;

    let _uint16: u16 = 32767;
    let _int16: i16 = -32768;

    let _uint32: u32 = 2147483647;
    let _int32: i32 = -2147483648;

    let _uint64: u64 = 9223372036854775807;
    let _int64: i64 = -9223372036854775808;

    let _uint128: u128 = 170141183460469231731687303715884105727;
    let _int128: i128 = -170141183460469231731687303715884105728;

    // scalar: integer pointer-sized
    let _isize_max: isize = isize::MAX;
    let _usize_min: usize = usize::MIN;

    // scalar: integer literal
    let _decimal_literal = 2020_00;
    let _hexadecimal_literal = 0x7E4;
    let _octal_literal = 0o3744;
    let _binary_literal = 0b1111_1100_1000;
    let _byte_literal = b'a';

    // scalar: floating point
    let _float_default_literal = 1.0;
    let _float64_literal: f64 = 2.0;
    let _float32_literal: f32 = 3.0;

    // scalar: numeric operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    // scalar: boolean
    let _bool_true_implicit = true;
    let _bool_false: bool = false;

    // scalar: unicode character
    let _char_implicit = 'z';
    let _char_explicit: char = 'ℤ';
    let _char_emoji: char = '😻';

    // compound: tuple
    let _tuple_implicit = (500, 6.4, 1);
    let tuple_explicit: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tuple_explicit;
    let _five_hundred = tuple_explicit.0;
    let six_point_four = tuple_explicit.1;
    let _one = tuple_explicit.2;

    println!("The value of y is: {}", y);
    println!("The value of six_point_four is: {}", six_point_four);

    // compound: array
    let _array_implicit = [1, 2, 3, 4, 5];
    let array_explicit: [i32; 5] = [1, 2, 3, 4, 5];
    let _array_fill_explicit: [i32; 5] = [0; 5];

    let first = array_explicit[0];
    let _second = array_explicit[1];

    println!("The value of first is: {}", first);
}
