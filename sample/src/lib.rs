use std::fs::File;

use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn init_logger() -> std::io::Result<()> {
    // ファイルを作成する
    let log = File::create("application.log")?;

    // ログ出力を設定する
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true) 
        .compact();
    
    tracing_subscriber::fmt()
        .event_format(format)
        .with_writer(log.with_max_level(tracing::Level::INFO))
        .init();
    
    Ok(())
}