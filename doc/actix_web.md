# [actix-web](https://docs.rs/actix-web/3.3.2/actix_web/?search=#modules)

Actix Webは、パワフルで実用的、かつ非常に高速なRust用のWebフレームワークです。

---

### actix_web::App

- Description

  Application builder - アプリケーションインスタンスを構築するためのbuilder パターンに従った構造体。

- Implementations

  ### actix_web::App::new

  - Description

    Application builderの作成。builderのようなパターンでアプリケーションを構成することができます。

---

### actix_web::HttpRequest

- Description

  HTTPリクエスト

- Implementations

  ### actix_web::HttpRequest::match_info

  - Description

    Pathパラメータへの参照を取得します。

    Paramsは、URLパラメータのコンテナです。可変セグメントは`{identifier}`という形式で指定され、識別子はリクエストハンドラの中で後から使用して、そのセグメントのマッチした値にアクセスすることができます。

  ### actix_web::HttpRequest::bind
  
  - Description
  
    バインドするソケットのアドレス
  
    複数のアドレスをバインドするために、このメソッドを複数回呼び出すことができます。

---

### actix_web::HttpServer

- Description

  HTTPサーバー。

  アプリケーションファクトリで新しいHTTPサーバを作成します。

- Implementations

  ### actic_web::HttpServer::new

  - Description

    アプリケーションファクトリで新規にhttpサーバを作成します。

  ### actix_web::HttpServer::bind

  - Description

    バインドするソケットのアドレス

    複数のアドレスをバインドするために、このメソッドを複数回呼び出すことができます。

  ### actix_web::HttpServer::run

  - Description

    コネクションの受信を開始します。

    このメソッドは、いくつものhttpワーカーを別々のスレッドで起動します。各アドレスに対して、このメソッドはループで`accept()`を行う別のスレッドを開始します。

    このメソッドは、ソケット・アドレスがバインドされていない場合や、`Actix`システムがまだ構成されていない場合には、パニックを起こします。

---

### actix_web::Responder

- Description

  HTTPレスポンスに変換できる型が実装するトレイト。

  このトレイトを実装した型は、ハンドラの戻り値の型として使用できます。



---

### actix_web::dev::Path

- Description

  リソースパスのマッチ情報

  リソースパスに可変パターンが含まれている場合、`Path`はそれを保存します。

- Implementations

  ### actix_web::dev::Path::get

  - Description

    マッチしたパラメータを型変換せずに名前で取得

---

### actix_web::middleware::Logger

- Description

  リクエストやレスポンスの情報を端末に記録するためのミドルウェアです。

  ロガーミドルウェアは、標準のログクレートを使用して情報を記録します。アクセスログを見るには、actix_webパッケージのロガーを有効にする必要があります。(`env_logger`など)

- Usage

  指定されたフォーマットでロガーのミドルウェアを作成します。デフォルトの`Logger`は、デフォルトのメソッドで作成することができ、デフォルトのフォーマットを使用します。

  ```
  %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
  ```

  ```rust
  use actix_web::{middleware::Logger, App};
  
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  
  let app = App::new()
      .wrap(Logger::default())
      .wrap(Logger::new("%a %{User-Agent}i"));
  ```

- Format

  `%%` パーセント記号

  `%a` リモートIPアドレス（リバースプロキシを使用する場合はプロキシのIPアドレス)

  `%t` リクエストの処理が開始された時間（rfc3339形式)

  `%r` 要求の最初の行

  `%s` 応答のステータスコード

  `%b` HTTP ヘッダを含む、バイト単位の応答のサイズ

  `%T` 要求を処理するのにかかった時間 (秒単位、0.06f 形式の浮動小数点)

  `%D` 要求を処理するのにかかった時間 (ミリ秒単位)

  `%U` リクエストのURL

  `%{r}a` 実際の IP リモート・アドレス *。

  `%{FOO}i`リクエスト.ヘッダ['FOO']

  `%{FOO}o` レスポンス.ヘッダ['FOO']

  `%{FOO}e` os.environ['FOO']

  `%{FOO}xi` [custom request replacement](https://docs.rs/actix-web/3.3.2/actix_web/middleware/struct.Logger.html#method.custom_request_replace) ラベル付き "FOO"

- Security

  * この値は[`ConnectionInfo::realip_remote_addr`](https://docs.rs/actix-web/3.3.2/actix_web/dev/struct.ConnectionInfo.html#method.realip_remote_addr)で計算されます。

  この値を使用する場合は、すべてのリクエストが信頼できるホストからのものであることを確認してください。なぜなら、リモートクライアントが別のクライアントであることを簡単に装うことができるからです。

---

### server

- Description

  Httpサーバモジュール

  このモジュールには、HTTPサーバの設定に必要なすべてのものが含まれています。

  HTTPサーバを起動するには、まず、`new`に用意されているファクトリを使用して、HTTPサーバの作成と設定を行う必要があります。

- Factory

  ファクトリーは、受信したHTTPリクエストにどのように対応するかを記述したApplicationを返す関数です。

  サーバーはワーカープールを使用しているため、ファクトリ関数はトレイト境界 の `Send + Clone + 'static'`に制限されており、各ワーカーは同期を必要とせずに Application を受け入れることができます。

  もし、すべてのワーカーで状態の一部を共有したい場合は、Arcでラップし、ラップされたタイプがスレッドセーフでない場合は、`RwLock`のような潜在的な同期プリミティブを使用する必要があります。

  ただし、ロックは非同期プログラミングにはお勧めできないので、リクエストハンドラ内でのロックは最小限にすべきです。

- HTTPS Support

  HTTPSサポート
  `Actix-web`では、TLSを提供する主要なクレートをサポートしています。それぞれのTLS実装には、HTTPサーバがどのように接続を受け付けるかを示す`AcceptorService`が用意されています。

  bindやlistenには、これらのサービスを受け入れる`bind_ssl|tls|rustls`や`listen_ssl|tls|rustls`があります。

  注：`native-tls`はまだHTTP2をサポートしていません。

- Signal handling and shutdown

  デフォルトでは、HTTPサーバはシステムシグナルを監視し、最大で30秒後にシャットダウンします。

  シグナルハンドリングとシャットダウンのタイムアウトは、対応するメソッドで制御できます。

  もしワーカーが何らかの理由でタイムアウト以内にシャットダウンできない場合、強制的にドロップされます。

- Example

  ```rust
  extern crate actix;
  extern crate actix_web;
  extern crate rustls;
  
  use actix_web::{http, middleware, server, App, Error, HttpRequest, HttpResponse, Responder};
  use std::io::BufReader;
  use rustls::internal::pemfile::{certs, rsa_private_keys};
  use rustls::{NoClientAuth, ServerConfig};
  
  fn index(req: &HttpRequest) -> Result<HttpResponse, Error> {
     Ok(HttpResponse::Ok().content_type("text/plain").body("Welcome!"))
  }
  
  fn load_ssl() -> ServerConfig {
     use std::io::BufReader;
  
     const CERT: &'static [u8] = include_bytes!("../cert.pem");
     const KEY: &'static [u8] = include_bytes!("../key.pem");
      
     let mut cert = BufReader::new(CERT);
     let mut key = BufReader::new(KEY);
  
     let mut config = ServerConfig::new(NoClientAuth::new());
     let cert_chain = certs(&mut cert).unwrap();
     let mut keys = rsa_private_keys(&mut key).unwrap();
     config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
  
     config
  }
  
  fn main() {
     let sys = actix::System::new("http-server");
     // load ssl keys
     let config = load_ssl();
  
      // create and start server at once
      server::new(|| {
          App::new()
              // register simple handler, handle all methods
              .resource("/index.html", |r| r.f(index))
              }))
      }).bind_rustls("127.0.0.1:8443", config)
      .unwrap()
      .start();
  
      println!("Started http server: 127.0.0.1:8080");
      //Run system so that server would start accepting connections
      let _ = sys.run();
  }
  ```



---

### server::new

- Description

  アプリケーション・ファクトリを使用して新しいhttpサーバを作成します。

  これは、`server::HttpServer::new()`メソッドのショートカットです。

- Example

  ```rust
  use actix_web::{server, App, HttpResponse};
  
  fn main() {
      let sys = actix::System::new("example");  // <- create Actix system
  
      server::new(
          || App::new()
              .resource("/", |r| r.f(|_| HttpResponse::Ok())))
          .bind("127.0.0.1:59090").unwrap()
          .start();
  
      sys.run();
  }
  ```

  



---

### resource

- Description

  リソースを特定のパスに設定します。

  リソースは可変のパスセグメントを持つことができます。たとえば、パスが `/a/{name}/c`のリソースは、`/a/b/c`、`/a/1/c`、`/a/etc/c`などのパスを持つすべての受信リクエストにマッチします。

  変数セグメントは`{identifier}`という形式で指定されます。識別子は、リクエストハンドラーの中で後から使用して、そのセグメントのマッチした値にアクセスすることができます。これは、`HttpRequest.match_info()`メソッドによって返される`Params`オブジェクト内の識別子を検索することによって行われます。

  デフォルトでは、各セグメントは正規表現`[^{}/]+`にマッチします。

  カスタムの正規表現を`{identifier:regex}`の形式で指定することもできます。

  たとえば、`/users/{userid}/{friend}`にマッチするルートでGETリクエストをルーティングし、公開されている`Params`オブジェクトに`userid`と`friend`を格納する場合などです。

- Example

  ```rust
  use actix_web::{http, App, HttpResponse};
  
  fn main() {
      let app = App::new().resource("/users/{userid}/{friend}", |r| {
          r.get().f(|_| HttpResponse::Ok());
          r.head().f(|_| HttpResponse::MethodNotAllowed());
      });
  }
  ```



---

### run

- Description

  新しいスレッドを生成し、受信する接続のリスニングを開始します。

  このメソッドは、新しいスレッドを生成し、新しい`actix`システムを起動します。それ以外は、`start()`メソッドと同様です。このメソッドはブロックします。

  このメソッドは、ソケット・アドレスがバインドされなかった場合、パニックを起こします。

- Example

  ```rust
  fn main() {
      HttpServer::new(|| App::new().resource("/", |r| r.h(|_| HttpResponse::Ok())))
          .bind("127.0.0.1:0")
          .expect("Can not bind to 127.0.0.1:0")
          .run();
  }
  ```

  

  