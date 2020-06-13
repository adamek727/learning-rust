enum MatchEnums {
    option1,
    option2,
    option3,
}

fn main() {


    // Match
    let s = MatchEnums::option2;
    match s {
        MatchEnums::option1 => println!("1"),
        MatchEnums::option2 => println!("2"),
        MatchEnums::option3 => println!("3"),
        _ => println!("Unexpected case"),
    }


    // if else
    let y = 5;
    if y < 3 {
        println!("Smaller than 3")
    } else if y > 3 {
        println!("bigger than 3")
    } else {
        println!("exactly 3")
    }



    // while let loop
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(x) = stack.pop() {
        println!("{}", x);
    }

    // For loop
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} : {}", index, value);
    }

    // Let statement
    let x = 5;
    let (a,b,c) = (1,2,3);
    let (i , _, k) = (2,3,4);

    // function parameters
    let point = (5, 10);
    foo(&point);
}


fn foo(&(x, y): &(i32, i32)) {

    println!("({}, {})",x ,y);
}