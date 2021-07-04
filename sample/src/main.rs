use std::sync::{Mutex, Arc};
use std::thread;

use log::info;

fn main() {
    env_logger::init();
    // `Arc`で値を内包した値の参照を作成し、スレッド間で共有する
    // 値を`Mutex`でラップすることで、共有した値を変更可能にする
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 値を変更するために、`lock`メソッドでロックを取得する
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        // サブスレッドの処理が終わるまで待機する
        handle.join().unwrap();
    }
    info!("Result: {}", counter.lock().unwrap());
}