use log::{Level, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::error::Error;
use std::path::Path;

pub type LoggerResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Configure logger instance.
/// The log output will be saved in `log_dir/output.log`.
///
/// * `log_dir` - Directory path to save output log.
/// * `level`   - Logging level.
///
/// # Examples
/// ```
/// use perception_eval::utils::logger::{configure_logger, LoggerResult};
/// use log::Level;
/// use std::path::Path;
///
/// fn main() -> LoggerResult<()> {
///     let log_dir = Path::new("work_dir/log");
///     configure_logger(&log_dir, Level::Info)?;
///     Ok(())
/// }
/// ```
pub fn configure_logger(log_dir: &Path, level: Level) -> LoggerResult<()> {
    let logfile: FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
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
