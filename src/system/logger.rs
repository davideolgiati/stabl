use chrono::DateTime;
use chrono::Utc;
use std::time::SystemTime;
use chrono::format::SecondsFormat;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error
}

lazy_static! {
    static ref LOGGER: Mutex<Vec<LoggingLevel>> = Mutex::new(Vec::new());
}

pub fn set_logging_level(level: LoggingLevel) {
        LOGGER.lock().unwrap().push(level);
}

pub fn get_logging_level() -> LoggingLevel {
        *LOGGER.lock().unwrap().last().unwrap_or(&LoggingLevel::Info)
}

pub fn get_current_ts_string() -> String {
        let current_ts = DateTime::<Utc>::from(SystemTime::now());
        current_ts.to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[macro_export]
macro_rules! trace {
        ($($args:tt)*) => {
                if($crate::system::logger::get_logging_level() == $crate::system::logger::LoggingLevel::Trace) {
                        print!(
                                "{:<24}  [\x1b[1mTRACE\x1b[0m]  ", 
                                $crate::system::logger::get_current_ts_string()
                        );
                        println!($($args)*);
                }
        }
}

#[macro_export]
macro_rules! debug {
        ($($args:tt)*) => {
                if($crate::system::logger::get_logging_level() <= $crate::system::logger::LoggingLevel::Debug) {
                        print!(
                                "{:<24}  [\x1b[97;1mDEBUG\x1b[0m]  ", 
                                $crate::system::logger::get_current_ts_string()
                        );
                        println!($($args)*);
                }
        }
}

#[macro_export]
macro_rules! info {
        ($($args:tt)*) => {
                if($crate::system::logger::get_logging_level()  <= $crate::system::logger::LoggingLevel::Info) {
                        print!(
                                "{:<24}  [\x1b[94;1mINFO\x1b[0m ]  ", 
                                $crate::system::logger::get_current_ts_string()
                        );
                        println!($($args)*);
                }
        }
}

#[macro_export]
macro_rules! warn {
        ($($args:tt)*) => {
                if($crate::system::logger::get_logging_level()  <= $crate::system::logger::LoggingLevel::Warn) {
                        print!(
                                "{:<24}  [\x1b[93;1mWARN\x1b[0m ]  ",
                                $crate::system::logger::get_current_ts_string()
                        );
                        println!($($args)*);
                }
        }
}

#[macro_export]
macro_rules! error {
        ($($args:tt)*) => {
                if($crate::system::logger::get_logging_level() <= $crate::system::logger::LoggingLevel::Error) {
                        print!(
                                "{:<24}  [\x1b[91;1mERROR\x1b[0m]  ",
                                $crate::system::logger::get_current_ts_string()
                        );
                        println!($($args)*);
                }
        }
}

#[macro_export]
macro_rules! start_timer {
        () => {
                std::time::SystemTime::now()
        }
}

#[macro_export]
macro_rules! stop_timer {
        ($start_time: expr) => {
                match $start_time.elapsed(){
                        Ok(elapsed) => {
                                elapsed.as_millis()
                        }
                        Err(e) => {
                                panic!("Error: {:?}", e)
                        }
                }
        }
}