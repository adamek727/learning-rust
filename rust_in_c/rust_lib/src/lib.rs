#[no_mangle]
pub extern "C" fn rust_function() {
    println!("This is RUST");
}

#[no_mangle]
pub extern "C" fn rust_acc_fnc(a: i32, b:i32) -> i32 {
    a + b
}