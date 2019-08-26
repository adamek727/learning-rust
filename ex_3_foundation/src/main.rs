
fn main() {

    // Variables, mut variables and const variables

    let mut x = 5;
    const Y: u32 = 5;

    println!("{}", x);

    x = 2;

    println!("{}", x);
    println!("{}", Y);

    // Shadowing

    let x = x+1;
    let x = x*x;

    println!("{}",x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // Scalars
    let number: u32 = 1_000_000;
    let negative_number: i16 = -654;
    let hex_number : usize = 0x987;
    let oct_number : isize = -0o155;
    let bin_number : u8 = 0b10101010;
    let byte_number : u8 = b'A';

    let float_64_val = 1e20;
    let float_32_val : f32 = 1e10;

    let bool_val = true;

    let char_val = 'A';

    // Numeric operations
    let sum_result = 1.0 + 1.0;
    let dif_result = 1 - 5;
    let div_result = 5.0 / 0.1;
    let mul_resutl = 10 * 5;
    let rem_result = 50 % 3;

    // Compound types - group of multiple values

    let explicit_tuple_val : (i32, f64, bool) = (666, 6.66, false);
    let implicit_tuple_val = (10, 20.0, true);
    let (x, y, z) = implicit_tuple_val;
    let tuple_pickup = implicit_tuple_val.0;
    println!("{} {} {} {}", x, y ,z, tuple_pickup);

    // Array

    let arr = [1, 2, 3, 4, 5];
    println!(" {} {} {}", arr[0], arr[1], arr[2]);
}

