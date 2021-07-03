use log::{trace, debug, info, warn, error};

fn main() {
    env_logger::init();
    let i = 100;
    trace!("trace {}", i);
    debug!("debug {}", i);
    info!("hello {}", i);
    warn!("warn {}", i);
    error!("error {}", i);
}
