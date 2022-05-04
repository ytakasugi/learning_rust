
use sample::{fibonacci, tribonacci};

fn init_logger(){
    let base_config = fern::Dispatch::new();

    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Error)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S.%H%M%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("error.log").unwrap());

    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S.%H%M%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("app.log").unwrap());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .unwrap();
}


fn main() {
    init_logger();

    // フィボナッチ数列、トリボナッチ数列の第4項を計算する。
    let n = 4;
    log::info!("try to calculate fibonacci({})", n);
    let fib = fibonacci::fibonacci(n);
    println!("fib[{}] = {}", n, fib);
    log::info!("try to calculate tribonacci({})", n);
    let trib = tribonacci::tribonacci(n);
    println!("trib[{}] = {}", n, trib);
}