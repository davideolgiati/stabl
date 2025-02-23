use std::str::FromStr;
use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub enum Severity {
        None,
        Low,
        Moderate,
        Important,
        Critical
}

impl FromStr for Severity {
        type Err = String;
    
        fn from_str(input: &str) -> Result<Severity, Self::Err> {
                match input.to_lowercase().as_str() {
                        "critical" => Ok(Severity::Critical),
                        "important" => Ok(Severity::Important),
                        "moderate" => Ok(Severity::Moderate),
                        "low" => Ok(Severity::Low),
                        "none" => Ok(Severity::None),
                        _               => Err(format!("'{}' is not a valid value for Severity", input)),
                }
        }
}

impl Display for Severity {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Critical  => write!(f, "critical"),
                Self::Important  => write!(f, "important"),
                Self::Moderate  => write!(f, "moderate"),
                Self::Low => write!(f, "low"),
                Self::None => write!(f, "none"),
            }
        }
    }
    