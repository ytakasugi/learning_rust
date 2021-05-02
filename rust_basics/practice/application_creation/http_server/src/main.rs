mod parser;

use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};
use std::io;

fn server_start() -> io::Result<()> {
    // 172.30.217.78:8080をlistenし、acceptしつづけるリスナーを作成
    // `?`後置演算子を使ってエラー時には単に関数から抜ける
    // このリスナーはResultで包まれている
    let lis = TcpListener::bind("172.30.217.78:8080")?;
    // コネクションがあるたびに`stream`を取り出す
    for stream in lis.incoming() {
        // streamを包むResultを一旦剥がす
        // エラーが出てもループを継続するために`?`は使わない
        let mut stream = match stream {
            Ok(stream) => stream,
            // acceptでエラーが起きたらそれを通知して次のループへ
            Err(e) => {
                println!("An error occurred while accepting a connection: {}", e);
                continue;
            }
        };

        // IO処理はブロックするので別スレッドを立てる
        // そうすることでリクエストを処理しつつ新たなコネクションを受け付けられる
        // スレッドはspawnしたあと見捨てるので返り値のスレッドハンドルは無視
        let _ = thread::spawn(
            // spawnの引数にはクロージャを渡す
            // クロージャは `|引数| 本体` あるいは `|引数| -> 返り値の型 { 本体 }` で作る
            // 関数と違って`()`でなくとも型を推論できるなら引数の型や`-> 返り値の型`を省略できる
            // `move`はクロージャが捕捉した変数（今回は`stream`）の所有権をクロージャにムーブするためのキーワード
            move || -> io::Result<()> {
                use parser::ParseResult::*;
                let mut buf = Vec::new();
                loop {
                    // 1回のread分を格納する一時バッファ
                    let mut b = [0; 1024];
                    // バッファにクライアントからの入力を読み込み
                    let n = stream.read(&mut b)?;
                    // 読み込んだバイト数が0ならストリームの終了（スレッドから抜ける）
                    if n == 0 {
                        return Ok(());
                    } 
                    // リクエスト全体のバッファに、読み込んだ分を追記
                    buf.extend_from_slice(&b[0..n]);
                    // それ以外ではHTTP/0.9のリクエストの処理
                    match parser::parse(buf.as_slice()) {
                        // 入力の途中なら新たな入力を待つため次のイテレーションへ
                        Partial => continue,
                        // エラーなら不正な入力なので何も返さずスレッドから抜ける
                        // スレッドから抜けると`stream`のライフタイムが終わるため、コネクションが自動で閉じられる
                        Error => {
                            return Ok(());
                        },
                        // リクエストが届けば処理をする
                        Complete(req) => {
                        // レスポンスを返す処理をここに書く
                        // 本来はファイルの中身を返すが、ここではリクエストの内容を含んだ文字列を返す
                        write!(stream, "OK {}\r\n", req.0)?;
                        // 処理が完了したらスレッドから抜ける
                        return Ok(());
                        },
                    };
                }
            }
        );
    }
    Ok(())
}


fn main() {
    match server_start() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}