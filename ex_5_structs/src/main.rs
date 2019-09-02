
#[derive(Debug)]
struct Rectange{
    width: u32,
    height: u32
}

impl Rectange {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    let a = 5;
    let b = 6;

    let rect = Rectange{width: a, height: b};

    println!("Rectangle: {:?}", rect);
    println!("Rectangle area is: {}", area_two_args(a,b));
    println!("Rectangle area is: {}", area_tuples((a,b)));
    println!("Rectangle area is: {}", area_rect_arg(&rect));
    println!("Rectangle area is: {}", rect.area());

}


fn area_two_args(a: u32, b: u32) -> u32{
    a * b
}

fn area_tuples( dim : (u32, u32)) -> u32 {
    dim.0 * dim.1
}
fn area_rect_arg( rect : &Rectange) -> u32 {
    rect.width * rect.height
}


