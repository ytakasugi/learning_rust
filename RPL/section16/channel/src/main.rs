use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // mpsc::channel関数はタプルを返し、1つ目の要素は、送信側、2つ目の要素は受信側になる
    let (tx, rx) = mpsc::channel();
    // チャンネルの送信側に対してcloneを呼び出している。 これにより、最初に立ち上げたスレッドに渡せる新しい送信ハンドルが得られる。
    //元のチャンネルの送信側は、2番目に立ち上げたスレッドに渡します。これにより2つスレッドが得られる 
    let tx1 = mpsc::Sender::clone(&tx);

    // thread::spawnを使用して新しいスレッドを生成し、それからmoveを使用して、 立ち上げたスレッドがtxを所有するようにクロージャにtxをムーブしている
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
