use sample::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger()?;
    // 論理コア数を表示
    tracing::info!("Logical cores: {}", num_cpus::get());

    let threads: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                tracing::info!("{}", i);
            })
        })
        .collect();

    for thread in threads {
        thread.await?;
    }

    Ok(())
}