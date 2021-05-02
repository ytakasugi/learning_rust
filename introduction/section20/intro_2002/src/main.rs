use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // チャネルには`Sender<T>`と`Receiver<T>`という2つのエンドポイントがある。
    // ここで、`T`は送信されるメッセージの型である。
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // 送信者エンドポイントはコピーすることができる。
        let thread_tx = tx.clone();

        // ここでは、それぞれのスレッドが自身のIDを送信している。
        let child = thread::spawn(move || {
            // スレッドは`thread_tx`の所有権をとり、それぞれのスレッドは
            // メッセージをチャネルにキューイングする。
            thread_tx.send(id).unwrap();

            // 送信はノンブロッキングなオペレーションなので、
            // メッセージを送信した後もすぐに実行を継続する。
            println!("thread {} finished", id);
        });
        children.push(child);
    }

    // ここで、全てのメッセージが収集される。
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    
    for _ in 0..NTHREADS {
        // `recv`メソッドはチャネルからメッセージを取り出す。
        // もし取り出せるメッセージが存在しない場合、`recv`は
        // 現在のスレッドをブロックする。
        ids.push(rx.recv());
    }

    // 残っている作業をスレッドが完了するのを待つ
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // メッセージが送信された順番を表示
    println!("{:?}", ids);
}
