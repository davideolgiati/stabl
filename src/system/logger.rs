#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error
}

pub struct Logger {
        level: LoggingLevel
}

impl Logger {
        pub fn new(level: LoggingLevel) -> Logger {
                Logger { level }
        }

        pub fn get_level(&self) -> LoggingLevel {
                self.level
        }
}

#[macro_export]
macro_rules! trace {
        ($logger:expr, $($args:tt)*) => {
                if($logger.get_level() == $crate::system::logger::LoggingLevel::Trace) {
                        print!(
                                "{:<35} [\x1b[1mTRACE\x1b[0m] ", 
                                chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::now()).to_rfc3339()
                        );
                        println!($($args)*);
                }
        }
}

#[macro_export]
macro_rules! debug {
        ($logger:expr, $($args:tt)*) => {
                if($logger.get_level() <= $crate::system::logger::LoggingLevel::Debug) {
                        print!(
                                "{:<35} [\x1b[97;1mDEBUG\x1b[0m] ", 
                                chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::now()).to_rfc3339()
                        );
                        println!($($args)*);
                }
        }
}

#[macro_export]
macro_rules! info {
        ($logger:expr, $($args:tt)*) => {
                if($logger.get_level() <= $crate::system::logger::LoggingLevel::Info) {
                        print!(
                                "{:<35} [\x1b[94;1mINFO\x1b[0m ] ", 
                                chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::now()).to_rfc3339()
                        );
                        println!($($args)*);
                }
        }
}

#[macro_export]
macro_rules! warn {
        ($logger:expr, $($args:tt)*) => {
                if($logger.get_level() <= $crate::system::logger::LoggingLevel::Warn) {
                        print!(
                                "{:<35} [\x1b[93;1mWARN\x1b[0m ] ",
                                chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::now()).to_rfc3339()
                        );
                        println!($($args)*);
                }
        }
}

#[macro_export]
macro_rules! error {
        ($logger:expr, $($args:tt)*) => {
                if($logger.get_level() <= $crate::system::logger::LoggingLevel::Error) {
                        print!(
                                "{:<35} [\x1b[91;1mERROR\x1b[0m] ",
                                chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::now()).to_rfc3339()
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