use std::thread;

use sample::init_logger;

const MAX_THREAD: i32 = 10;

#[tokio::main]
async fn main() {
    init_logger();
    // 論理コア数を表示
    log::info!("Logical cores: {}", num_cpus::get());

    for _ in 0..MAX_THREAD {
        tokio::spawn(async {
            log::info!("[{:?}]", thread::current().id());
        });
    }
    log::info!("[{:?}]", thread::current().id());
}