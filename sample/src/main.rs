use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Rust"),
            String::from("is"),
            String::from("a"),
            String::from("well"),
            String::from("loved"),
            String::from("language")
        ];

        for i in vals {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for r in rx {
        println!("Got: {}", r);
    }
}