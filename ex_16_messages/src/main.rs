use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {

    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn( move || {
        let vals = vec![
            String::from("hi"),
            String::from("super"),
            String::from("hero"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn( move || {
        let vals = vec![
            String::from("hi2"),
            String::from("super2"),
            String::from("hero2"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    
    for received in rx {
        println!("Got: {}", received);
    }

}
