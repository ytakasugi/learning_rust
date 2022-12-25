use std::thread;

pub fn init_logger(){
    let base_config = fern::Dispatch::new();

    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Error)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}][{:?}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S.%H%M%S]"),
                record.level(),
                record.target(),
                thread::current().id(),
                message
            ))
        })
        .chain(fern::log_file("error.log").unwrap());

    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}][{:?}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S.%H%M%S]"),
                record.level(),
                record.target(),
                thread::current().id(),
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