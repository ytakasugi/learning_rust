use std::env;

fn arguments(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        // `Option::ok_or`コンビネータを使用して、`Option<T>`を`Result<T, E>`に変換
        .ok_or("Please give at least one argument".to_owned())
        // `arg.parse::<i32>`が返す`Result<i32, ParseIntError`>を
        // `Result::map_err`コンビネータで`Result<i32, String>`へ変換
        .and_then(|arg| arg.parse::<i32>()
            .map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn main() {
    match arguments(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}