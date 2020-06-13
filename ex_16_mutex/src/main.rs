use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::rc::Rc;

fn main() {

    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter_copy = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter_copy.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
