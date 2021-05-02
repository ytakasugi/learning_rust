#![allow(unused)]
fn main() {
    use std::thread;
    use std::time::Duration;

    // Fnトレイトを使用することで、クロージャであると指定している
    struct Cacher<T>
        where T: Fn(u32) -> u32 
    {
        calcuation: T,
        // クロージャを実行する前は、`value`は`None`になる
        // Cacherを使用するコードが結果を求めてきたときにクロージャを実行し、
        // その結果をvalueフィールドのSome列挙子に保存する
        // その後、コードが再度クロージャの結果を求めたら、クロージャを再実行するのでなく
        // CacherのSome列挙子に保持された結果を返却する
        value: Option<u32>,
    }
    

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32
    {
        // calcuationフィールドに指定されたクロージャとvalueフィールドに保持するNone値を保持するCacherインスタンスを
        // Cacher::newは返却する
        fn new(calcuation: T) -> Cacher<T> {
            Cacher {
                calcuation,
                value: None,
            }
        }
        // 呼び出し元のコードがクロージャの評価結果を必要としたとき、クロージャを直接呼ぶ代わりにvalueメソッドを呼び出す
        // このメソッドは、結果の値が`self.value`にあるか確認する
        // 結果があれば、クロージャを再度実行することなく、Some内の値を返却する
        // `self.value`がNoneなら、コードはself.calcuationに保存されたクロージャを呼び出し、結果を`self.value`に
        // 保存し、その値を返す
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calcuation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        } 
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        // クロージャを変数に直接保存する代わりに、クロージャを保持するCacherの新規インスタンスを保存にしている
        // 結果が必要な箇所それぞれで、Cacherインスタンスに対してvalueメソッドを呼び出す
        let mut expensive_result = Cacher::new(|num|  {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }

    // クロージャのサンプルコード
    // xを受け取り、その二乗を返却するクロージャ
    // 型が自明な場合、型注釈は不要
    // 型注釈をつける場合、`|x: i32| {..}`といったようにする
    let square = |x| {
        x * x
    };
    println!("{}", square(9));

    // クロージャ外変数msg
    let msg = String::from("Hello World");
    // `move`を使用すると、変数msgの所有権がクロージャに移動する
    let func = move || {
        println!("{}", msg);
    };
    // クロージャを呼び出す。クロージャ終了時にライフタイムが終了するので、以降、変数msgを参照しようとするとエラーとなる
    func();
}
