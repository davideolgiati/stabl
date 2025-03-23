use std::str::FromStr;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
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
        assert!(!input.is_empty());

        match &*input.to_lowercase() {
            "critical"      => Ok(Severity::Critical),
            "important"     => Ok(Severity::Important),
            "moderate"      => Ok(Severity::Moderate),
            "low"           => Ok(Severity::Low),
            "none"          => Ok(Severity::None),
            _               => Err(format!("'{}' is not a valid value for Severity", input)),
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Critical   => write!(f, "CRITICAL"),
            Self::Important  => write!(f, "IMPORTANT"),
            Self::Moderate   => write!(f, "MODERATE"),
            Self::Low        => write!(f, "LOW"),
            Self::None       => write!(f, "NONE"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn happy_path_severity_critical() {
        let input: &str = "critical";
        let expected: Severity = Severity::Critical;
        let output = Severity::from_str(input).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_severity_important() {
        let input: &str = "important";
        let expected: Severity = Severity::Important;
        let output = Severity::from_str(input).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_severity_moderate() {
        let input: &str = "moderate";
        let expected: Severity = Severity::Moderate;
        let output = Severity::from_str(input).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_severity_low() {
        let input: &str = "low";
        let expected: Severity = Severity::Low;
        let output = Severity::from_str(input).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_severity_none() {
        let input: &str = "none";
        let expected: Severity = Severity::None;
        let output = Severity::from_str(input).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn print_critical() {
        let input: &str = "critical";
        let expected: &str = "CRITICAL";
        let output = Severity::from_str(input).unwrap();

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    fn print_important() {
        let input: &str = "important";
        let expected: &str = "IMPORTANT";
        let output = Severity::from_str(input).unwrap();

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    fn print_moderate() {
        let input: &str = "moderate";
        let expected: &str = "MODERATE";
        let output = Severity::from_str(input).unwrap();

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    fn print_low() {
        let input: &str = "low";
        let expected: &str = "LOW";
        let output = Severity::from_str(input).unwrap();

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    fn print_none() {
        let input: &str = "none";
        let expected: &str = "NONE";
        let output = Severity::from_str(input).unwrap();

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    fn panic_unknown_string() {
        let input: &str = "major";
        let output = Severity::from_str(input);

        assert!(output.is_err(), "'major' is not a valid value for Severity");
    }

    #[test]
    #[should_panic]
    fn panic_empty_string() {
        Severity::from_str("").unwrap();
    }
}
    