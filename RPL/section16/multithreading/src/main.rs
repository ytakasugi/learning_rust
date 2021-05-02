use std::thread;
use std::time::Duration;

fn main() {
    // thread::spawn関数を呼び出し、新規にスレッドを生成し実行したいコードを含むクロージャを渡している
    // 引数なしのクロージャは、forループで生成した値を返却し、thread::spawn関数はクロージャから返却された値を引数として実行している
    // thread::spawnの戻り値を変数に保存する。thread::spawnの戻り値の型はJoinHandleである
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // メインスレッド
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // JoinHandleは、そのjoinメソッドを呼び出したときにスレッドの終了を待つ所有された値である
    // メインスレッドの前に下記のコードを配置すると、メインスレッドの前に立ち上げたスレッドを実行する
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    // thread::spawnに渡しているクロージャには引数がなく 立ち上げたスレッドのコードでメインスレッドからのデータは何も使用していない
    // 立ち上げたスレッドで、スレッド外のデータを使用するには`move`を使用して`v`をキャプチャし、クロージャの環境の一部にしている
    // `move`でクロージャに使用している値の所有権を強制的に奪わせている
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();
}
