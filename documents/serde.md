### Crate [serde](https://docs.serde.rs/serde/index.html)

`Serde`は、Rust データ構造を効率的かつ汎用的にシリアル化/デシリアル化するためのフレームワークです。

`Serde`のエコシステムは、自分自身をシリアル化/デシリアル化する方法を知っているデータ構造と、他のものをシリアル化/デシリアル化する方法を知っているデータフォーマットで構成されています。`Serde`は、この 2 つのグループが相互に作用する層を提供し、サポートされているあらゆるデータ構造を、サポートされているあらゆるデータフォーマットを使用してシリアル化およびシリアル化解除できるようにします。

追加のドキュメントや使用例については、[`Serde`のウェブサイト](https://serde.rs/)を参照してください。

---

### Design

他の多くの言語では、データのシリアライズをランタイムリフレクションに頼っていますが、`Serde`は代わりに Rust の強力なトレイトシステムに基づいて構築されています。自分自身をシリアライズ/デシリアライズする方法を知っているデータ構造は、`Serde`の`Serialize`および`Deserialize`トレイトを実装している (または`Serde`の`derive`属性を使用してコンパイル時に実装を自動生成している) データ構造です。これにより、リフレクションやランタイム型情報のオーバーヘッドを回避できます。実際、多くの状況において、データ構造とデータフォーマットの間の相互作用は、Rustコンパイラによって完全に最適化され、`Serde`シリアライズは、データ構造とデータフォーマットの特定の選択に対して、手書きのシリアライザと同じ速度で実行されます。

---

### Data formats

以下は、コミュニティによって`Serde`に実装されたデータフォーマットの一部です。

- [JSON](https://github.com/serde-rs/json)：多くのHTTP APIで使用されているユビキタスなJavaScript Object Notation。
- [Bincode](https://github.com/servo/bincode)：Servoレンダリングエンジン内のIPCに使用されるコンパクトなバイナリフォーマット。
- [CBOR](https://github.com/pyfisch/cbor)：バージョンネゴシエーションを必要としない小さなメッセージサイズのために設計された簡潔なバイナリオブジェクト表現。
- [YAML](https://github.com/dtolnay/serde-yaml)マークアップ言語ではないが人に優しい設定言語と自称しています。
- [MessagePack](https://github.com/3Hren/msgpack-rust)：コンパクトなJSONに似た効率的なバイナリフォーマット。
- [TOML](https://github.com/alexcrichton/toml-rs)：Cargoで使用されている最小限の設定フォーマットです。
- [Pickle](https://github.com/birkenfeld/serde-pickle)：Pythonの世界では一般的なフォーマットです。
- [RON](https://github.com/ron-rs/ron)：Rusty Object Notation（ラスティ・オブジェクト・ノーテーション）の略。
- [RON](https://github.com/ron-rs/ron)：MongoDBで使用されているデータストレージとネットワーク転送のフォーマット。
- [Avro](https://github.com/flavray/avro-rs)：Apache Hadoopで使用されているバイナリフォーマットで、スキーマ定義をサポートしています。
- [JSON5](https://github.com/callum-oakley/json5-rs)：ES5の一部を含むJSONのスーパーセットです。
- [Postcard](https://github.com/jamesmunns/postcard)：`no_std`および組み込みシステムに適したコンパクトなバイナリフォーマット。
- [URL](https://docs.rs/serde_qs)：`x-www-form-urlencode`形式のURLクエリ文字列です。
- [Envy](https://github.com/softprops/envy)：環境変数を Rust の構造体にデシリアライズする方法です。(デシリアライズのみ)
- [Envy Store](https://github.com/softprops/envy-store)：[AWS Parameter Store](https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-paramstore.html) パラメータを Rust 構造体にデシリアライズする方法です。(デシリアライズのみ)
- [S-expressions](https://github.com/rotty/lexpr-rs)： Lisp言語ファミリーで使用されているコードとデータのテキスト表現です。
- [D-Bus](https://docs.rs/zvariant)：バイナリワイヤフォーマット。
- [FlexBuffers](https://github.com/google/flatbuffers/tree/master/rust/flexbuffers)：GoogleのFlatBuffersのゼロコピーシリアル化フォーマットのスキームレスの従兄弟です。
- [Bencode](https://github.com/P3KI/bendy)：BitTorrent プロトコルで使用されるシンプルなバイナリ形式です。
- [DynamoDB Items](https://docs.rs/serde_dynamo)：[`rusoto_dynamodb`](https://docs.rs/rusoto_dynamodb)がDynamoDBとの間でデータを転送する際に使用するフォーマット。
- [Hjson](https://github.com/Canop/deser-hjson)：人間による読み取りと編集を目的としたJSONの拡張構文です。(デシリアライズのみ)