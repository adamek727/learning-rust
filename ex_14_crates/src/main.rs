extern crate ex_14_crates;


/// Adds one to the number in argument
///
/// # Example
///
/// let x = 10;
/// assert_qe!(6, ex_14_crates::add_one(x));
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}

use ex_14_crates::PrimaryColor;


fn main() {

    let p1 =  PrimaryColor::Red;

    let c1 = ex_14_crates::kinds::PrimaryColor::Blue;
    let c2 = ex_14_crates::kinds::PrimaryColor::Blue;

    let c3 = ex_14_crates::utils::mix(c1, c2);

    println!("Hello, world!");
}
