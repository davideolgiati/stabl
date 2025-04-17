use std::fmt::{self, Display, Formatter};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum SecurityClassification {
    None,
    Low,
    Moderate,
    Important,
    Critical
}

impl<'a> From<&'a str> for SecurityClassification {
    fn from(input: &'a str) -> Self {
        assert!(!input.is_empty());

        match &*input.to_lowercase() {
            "critical"      => SecurityClassification::Critical,
            "important"     => SecurityClassification::Important,
            "moderate"      => SecurityClassification::Moderate,
            "low"           => SecurityClassification::Low,
            "none"          => SecurityClassification::None,
            _               => panic!("'{}' is not a valid value for SecurityClassification", input),
        }
    }
}

impl Display for SecurityClassification {
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
        let expected: SecurityClassification = SecurityClassification::Critical;
        let output = SecurityClassification::from(input);

        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_severity_important() {
        let input: &str = "important";
        let expected: SecurityClassification = SecurityClassification::Important;
        let output = SecurityClassification::from(input);

        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_severity_moderate() {
        let input: &str = "moderate";
        let expected: SecurityClassification = SecurityClassification::Moderate;
        let output = SecurityClassification::from(input);

        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_severity_low() {
        let input: &str = "low";
        let expected: SecurityClassification = SecurityClassification::Low;
        let output = SecurityClassification::from(input);

        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_severity_none() {
        let input: &str = "none";
        let expected: SecurityClassification = SecurityClassification::None;
        let output = SecurityClassification::from(input);

        assert_eq!(output, expected);
    }

    #[test]
    fn print_critical() {
        let input: &str = "critical";
        let expected: &str = "CRITICAL";
        let output = SecurityClassification::from(input);

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    fn print_important() {
        let input: &str = "important";
        let expected: &str = "IMPORTANT";
        let output = SecurityClassification::from(input);

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    fn print_moderate() {
        let input: &str = "moderate";
        let expected: &str = "MODERATE";
        let output = SecurityClassification::from(input);

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    fn print_low() {
        let input: &str = "low";
        let expected: &str = "LOW";
        let output = SecurityClassification::from(input);

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    fn print_none() {
        let input: &str = "none";
        let expected: &str = "NONE";
        let output = SecurityClassification::from(input);

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    #[should_panic]
    fn panic_unknown_string() {
        let input: &str = "major";
        let _ = SecurityClassification::from(input);
    }

    #[test]
    #[should_panic]
    fn panic_empty_string() {
        let _ = SecurityClassification::from("");
    }
}
    