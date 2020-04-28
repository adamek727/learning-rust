extern crate add_one;

fn main() {
    let x = 5;

    let x2 = add_one::add_one(x);

    println!("{}", x2);
}
