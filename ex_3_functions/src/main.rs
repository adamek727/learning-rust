fn print_hello_fnc() {

    println!("Hello!");
}

fn function_with_arguments(x: i32, y: f64) {

    println!("The input agruments are {} and {}", x, y );
}


fn forty_two() -> i32 {
    42 // ret value (expression) is missing the semicolon
}


fn condition_function(x: i32) {

    if x < 42 {
        println!("argument is samller than 42");
    } else if x == 42 {
        println!("argument is eq to 42");
    } else {
        println!("argument is bigger than 42");
    }
}


fn if_in_let_statement_fnc(condition: bool) -> i32 {
    let x = {
        if condition {
            1
        } else {
            -1
        }
    };
    x
}


fn simple_loop_fnc() {
    let mut x = 3;
    loop {
        println!("loop iteration");
        if x == 0 {
            break;
        } else {
            x -= 1;
        }
    }
}


fn while_loop_fnc() {

    let arr = [1,2,3,4,5];
    let mut cnt = 0;
    while cnt < 5 {
        println!("Still running while loop: {}",arr[cnt]);
        cnt += 1;
    }
}


fn range_fnc() {

    for number in (1..4).rev() {
        println!("range: {}", number);
    }
}


fn for_iterator() {
    let arr = [10,20,30,40,50];
    for number in arr.iter() {
        println!("for_iterator fnc: {}",number);
    }
}


fn fibonacci (n: i32) -> i32 {
    let result = if n == 0 || n == 1 {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    };
    result
}

fn main() {
    print_hello_fnc();
    function_with_arguments(42, 42.0);

    let _x = 5;
    let y = {
        let x = 3;
        x + 1 // expression is missing semicolon at the end
    };
    println!("Result of the expression is: {}", y);

    println!("value forty-two by the function: {}", forty_two());

    condition_function(57);

    println!("condition let statement: {} and {}", if_in_let_statement_fnc(true), if_in_let_statement_fnc(false));

    simple_loop_fnc();
    while_loop_fnc();
    for_iterator();
    range_fnc();

    let n = 6;
    println!("Fibonacci fo n={} is {}", n, fibonacci(n));
}
