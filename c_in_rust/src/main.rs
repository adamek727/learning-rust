#[link(name = "matlib", kind = "static")]
extern "C" {
    fn add_fnc(a: f32, b: f32) -> f32;
}

fn main() {
    println!("Hello, world!");
    let res = unsafe{ add_fnc(15.0, 20.0) };
    println!("output: {}", res);
}
