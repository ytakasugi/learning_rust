use reqwest::Client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    // 空の`Client`インスタンスを作成
    let client = Client::new(); 
    let url = "https://zipcloud.ibsnet.co.jp/api/search";
    let response = client
        .get(url)
        // URLのクエリ文字列に追記する。
        // この例だと、「https://zipcloud.ibsnet.co.jp/api/search」が
        //「https://zipcloud.ibsnet.co.jp/api/search?zipcode=7830060」となる
        .query(&[("zipcode", "1000002")])
        // Requestを構築し、ターゲットのURLを送信してResponseを返す
        .send()
        .await?;
    // Responseの完全なテキストを取得
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}
