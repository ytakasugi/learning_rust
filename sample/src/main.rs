use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let handle = thread::spawn(move || {
        for i in v.iter() {
            println!("elem: {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    handle.join().unwrap();
}