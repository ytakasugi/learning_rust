extern crate multi_thread_server;
use multi_thread_server::ThreadPool;

use std::thread;
use std::time::Duration;
// std::io::preludeをスコープに導入して、ストリームから読み書きさせてくれる特定のトレイトにアクセスできるようにしている
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("172.31.80.158:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}
// TcpStreamインスタンスが内部で返すデータを追いかけるため、stream引数は可変としている
// 内部の状態が変化する可能性があるので、 mutにする必要がある
fn handle_connection(mut stream: TcpStream) {
    // スタックに読み取ったデータを保持するbufferをサイズ1024byteで宣言
    let mut buffer = [0; 1024];
    // bufferをstream.readに渡し、これがTcpStreamからbyteを読み取ってbufferに置く
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // bufferの先頭がgetならtrue
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "../../index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "../../index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "../../404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    // 成功したメッセージのデータを保持するresponse変数を定義
    let response = format!("{}{}", status_line, contents);

    // as_bytesを呼び出し、文字列データをバイトに変換
    // streamのwriteメソッドは、 &[u8]を取り、コネクションに直接そのバイトを送信
    stream.write(response.as_bytes()).unwrap();
    // flushは待機し、 バイトが全てコネクションに書き込まれるまでプログラムが継続するのを防ぐ
    stream.flush().unwrap();
}