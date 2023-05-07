use log::{Level, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::error::Error;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn configure_logger(log_dir: &Path, level: Level) -> Result<()> {
    let logfile: FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}{n}\n")))
        .build(log_dir.join("output.log"))?;

    let level_filter: LevelFilter = match level {
        Level::Debug => LevelFilter::Debug,
        Level::Info => LevelFilter::Info,
        Level::Warn => LevelFilter::Warn,
        Level::Error => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let config: Config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(level_filter))?;

    log4rs::init_config(config)?;

    Ok(())
}
