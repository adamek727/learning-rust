
struct Point {
    x: i32,
    y: i32,
}

enum Action {
    ChangeColor(i32, i32, i32),
    ChangeValue(f32, f64),
}


fn main() {

    let x = 1;

    match x {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Default"),
    }


    let x = Some(5);
    let y = 10;

    match x {
        Some(666) => println!("666"),
        Some(y) => println!("y: {}",y),
        _ => println!("Default"),
    }


    let x = 'k';
    match x {
        'a' ... 'i' => println!("early alphabet"),
        'j' ... 'z' => println!("later alphabet"),
        _ => println!("Out of alphabet"),
    }


    let p = Point { x: 5, y: 0};
    let Point {x: a, y:b} = p;
    println!("a: {}, b:{}", a, b);

    match p {
        Point {x: 0, y} => println!("On Y axis"),
        Point {x, y: 0} => println!("On X axis"),
        Point {x,y} => println!("Not on axis"),
    }


    let action = Action::ChangeColor(1, 2, 3);
    match action {
        Action::ChangeColor(r, g, b) => println!("r: {}, g: {}, b: {}", r, g ,b),
        Action::ChangeValue(val1, val2) => println!("val1: {}, val2: {}", val1, val2),
    }


    let points = vec![
        Point {x: 0, y: 10},
        Point {x: 6, y: 4},
        Point {x: 2, y: 8},
    ];
    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point {x, y}| x*x + y*y)
        .sum();
    println!("{}",sum_of_squares);



    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let p = Point3D{x: 1, y: 2, z:3};
    match p {
        Point3D{x, ..} => println!("x: {}", x),
    }

    let t = (1,2,3,4,5);
    match t {
        (first, .., fifth) => println!("{} {}", first, fifth),
    }



    let robot_name = Some(String::from("Boris"));
    match robot_name {
        Some(ref name) => println!("Name: {}", name),
        None => (),
    }
    println!("name: {}", robot_name.unwrap());



    enum Message{
        Hello {id: i32},
    }
    let msg = Message::Hello {id: 15};
    match msg {
        Message::Hello {id: id_variable @ 3...7} => println!("In range 3 to 7"),
        Message::Hello {id: id_variable @ 7...12} => println!("In range 7 to 12"),
        Message::Hello {id: id_variable} => println!("Out of range: {}", id_variable),
    }
}
