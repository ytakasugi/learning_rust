### std::net

---

#### std::net::TcpListener

- Description

  TCP ソケットサーバで、接続をリッスンします。

  ソケットアドレスにバインドしてTcpListenerを作成した後、着信TCP接続をリッスンします。これらは accept を呼び出すか、incoming で返された Incoming イテレータを反復処理することで受け入れることができます。

  値がドロップされるとソケットは閉じられます。

  送信制御プロトコルはIETF RFC 793で規定されています。

  - Example

~~~rust
    use std::net::{TcpListener, TcpStream};
    
    fn handle_client(stream: TcpStream) {
        // ...
    }
    
    fn main() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:80")?;
    
        // accept connections and process them serially
        for stream in listener.incoming() {
            handle_client(stream?);
        }
        Ok(())
    }
~~~

    

- Implementations

  - bind

    指定されたアドレスにバインドされる新しいTcpListenerを作成します。

    返されたリスナーは、接続を受け入れる準備ができています。

    ポート番号0でバインドすると、OSがこのリスナーにポートを割り当てるように要求します。割り当てられたポートは、`TcpListener::local_addr`メソッドで問い合わせることができます。

    アドレス型は`ToSocketAddrs`トレイトの任意の実装を指定することができます。具体的な例については、そのドキュメントを参照してください。

    `addr`が複数のアドレスを生成した場合、1つのアドレスが成功してリスナーを返すまで、それぞれのアドレスでバインドが試みられます。どのアドレスもリスナーの作成に成功しなかった場合、最後の試行 (最後のアドレス) から返されるエラーが返されます。

    

- Example

127.0.0.0.1:80 にバインドされた TCP リスナーを作成します。

~~~rust
      use std::net::TcpListener;
      
      let listener = TcpListener::bind("127.0.0.1:80").unwrap();
~~~

127.0.0.0.1:80 にバインドされた TCP リスナーを作成します。失敗した場合は、127.0.0.0.1:443 にバインドされた TCP リスナーを作成します。

~~~rust
      use std::net::{SocketAddr, TcpListener};
      
      let addrs = [
          SocketAddr::from(([127, 0, 0, 1], 80)),
          SocketAddr::from(([127, 0, 0, 1], 443)),
      ];
      let listener = TcpListener::bind(&addrs[..]).unwrap();
~~~

  - incoming

    このリスナーで受信している接続のイテレータを返します。

    返されるイテレータは `None`を返すことはなく、相手の`SocketAddr`構造体も返しません。これを繰り返し処理することは、ループ内で`TcpListener::accept`を呼び出すことと同じです。

    - Example

~~~rust
      use std::net::TcpListener;
      
      let listener = TcpListener::bind("127.0.0.1:80").unwrap();
      
      for stream in listener.incoming() {
          match stream {
              Ok(stream) => {
                  println!("new client!");
              }
              Err(e) => { /* connection failed */ }
          }
      }
~~~



---

#### std::net::TcpStream

- Description

  ローカルとリモートのソケット間のTCPストリーム。

  リモートホストに接続するか、TcpListener上で接続を受け付けるかのいずれかでTcpStreamを作成した後、そこに読み書きすることでデータを送信することができます。

  値をドロップした時点で接続を終了します。また、接続の読み書き部分は、シャットダウンメソッドで個別にシャットダウンすることができます。

  伝送制御プロトコルはIETF RFC 793に規定されています。

  - Example

~~~rust
    use std::io::prelude::*;
    use std::net::TcpStream;
    
    fn main() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:34254")?;
    
        stream.write(&[1])?;
        stream.read(&mut [0; 128])?;
        Ok(())
    } // the stream is closed here
~~~
