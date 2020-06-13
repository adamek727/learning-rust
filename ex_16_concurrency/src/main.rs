use std::thread;
use std::time::Duration;

fn main() {

    let v = vec![1, 2, 3];

    let thread_handler = thread::spawn(move || {
        println!("vector: {:?}", v)
    });

    thread_handler.join().unwrap();

    for i in 1..5 {
        println!("Hi from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

}
