extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // 文字列の出力
    println!("Guess the number!");
    // 1～101の間で乱数を取得
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);
    // ループさせ、正しい予想となった場合に終了する
    loop {
        println!("Please input your guess.");
        // letを使用して変数guessを生成。また、mutを使用して変数guessを可変変数にする。
        // String::new():String型のオブジェクトを返却。サイズは可変であり、UTF-8エンコードされたテキスト 
        // new関数で、新規に空の文字列を生成
        let mut guess = String::new();
        // io型のstdin関連関数を呼び出し、read_lineメソッドを呼び出してユーザーから入力を受けつける。
        // read_lineメソッドに対して、&mut guessという引数を渡している。
        // 「&」は、引数が参照であることを示す。
        // このため、データを複数回メモリにコピーせずに、複数個所で同じデータにアクセス可能
        // read_lineメソッドがエラーを返却した際、expectメソッドでクラッシュさせる
        io::stdin().read_line(&mut guess).expect("Faild to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);
        // matchで乱数で生成した値と、標準入力された値を比較
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}