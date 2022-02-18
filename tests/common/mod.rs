use simple_logger::SimpleLogger;

pub fn init_logger() {
    let _ = SimpleLogger::new().with_utc_timestamps().init();
}
