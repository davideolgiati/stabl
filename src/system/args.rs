use crate::model::semantic_version::SemanticVersion;
use crate::system::logger;

#[inline]
fn convert_release(arg: &str) -> SemanticVersion {
        match arg {
                "patch"  => SemanticVersion::Patch,
                "repack" => SemanticVersion::Repack,
                "minor"  => SemanticVersion::Minor,
                "major"  => SemanticVersion::Major,
                _ => { 
                        crate::error!("Invalid release type - {}", arg);
                        std::process::exit(1)
                }
        }
}

#[inline]
fn convert_verbosity(arg: &str) -> logger::LoggingLevel {
        let parsed_arg = arg.replace("--loglevel=", "");
        match parsed_arg.as_str() {
                "trace"  => logger::LoggingLevel::Trace,
                "debug" => logger::LoggingLevel::Debug,
                "info"  => logger::LoggingLevel::Info,
                "warn"  => logger::LoggingLevel::Warn,
                "error"  => logger::LoggingLevel::Error,
                _ => { 
                        crate::error!("Invalid verbosity level - {}", arg);
                        std::process::exit(1)
                }
        }
}

pub fn get_release_arg(release_args: &[String]) -> SemanticVersion {
        let release_arg: &str = {
                if release_args.len() >= 2 {
                        &release_args[1]
                } else {
                        ""
                }
        };

        convert_release(release_arg)
}

pub fn get_verbosity_arg(args: &[String]) -> logger::LoggingLevel {
        let verbosity_args: Vec<String> = args.iter()
                .filter(|arg| arg.starts_with("--loglevel="))
                .cloned()
                .collect();

        if verbosity_args.is_empty() {
                return logger::LoggingLevel::Info
        }

        convert_verbosity(verbosity_args.last().unwrap())
}

pub fn get_skip_security_updates_arg(args: &[String]) -> bool {
        let verbosity_args: Vec<String> = args.iter()
                .filter(|arg| arg.starts_with("--skip-security-updates"))
                .cloned()
                .collect();

        verbosity_args.is_empty()
}

pub fn look_for_help(args: &[String]) {
        if args.contains(&"--help".to_string()){
                let help:&str = r"          
Help:
        stabl major               only major and lower release are considered valid
        stabl minor               only minor and lower release are considered valid
        stabl patch               only patch and lower release are considered valid (default)
        stabl repack              only repack and lower release are considered valid
        
        --help                    display this help
        --loglevel=<level>        set satbl verbosity. valid options are: trace, debug, info, warn, error
        --skip-security-updates   skip security updates check
        ";
                println!("{}", help);
                std::process::exit(0)
        }
}

#[cfg(test)]
mod tests {
    use crate::system::logger::LoggingLevel;

    use super::*;
    
    #[test]
    fn happy_path_get_release_arg_patch() {
        let expected: SemanticVersion = SemanticVersion::Patch;
        let input: &[String] = &["stabl".to_string(), "patch".to_string()];

        let output = get_release_arg(input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_get_release_arg_repack() {
        let expected: SemanticVersion = SemanticVersion::Repack;
        let input: &[String] = &["stabl".to_string(), "repack".to_string()];
    
        let output = get_release_arg(input);
    
        assert!(expected == output)
    }

    #[test]
    fn happy_path_get_release_arg_minor() {
        let expected: SemanticVersion = SemanticVersion::Minor;
        let input: &[String] = &["stabl".to_string(), "minor".to_string()];

        let output = get_release_arg(input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_get_release_arg_major() {
        let expected: SemanticVersion = SemanticVersion::Major;
        let input: &[String] = &["stabl".to_string(), "major".to_string()];

        let output = get_release_arg(input);

        assert!(expected == output)
    }

    #[test]
    #[should_panic]
    fn should_panic_get_release_arg() {
        let input: &[String] = &["stabl".to_string(),  "bugfix".to_string()];
        get_release_arg(input);
    }

    #[test]
    fn happy_path_get_verbosity_arg_trace() {
        let expected: LoggingLevel = LoggingLevel::Trace;
        let input = vec!["patch".to_string(), "--loglevel=trace".to_string()];

        let output = get_verbosity_arg(&input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_get_verbosity_arg_debug() {
        let expected: LoggingLevel = LoggingLevel::Debug;
        let input = vec!["patch".to_string(), "--loglevel=debug".to_string()];

        let output = get_verbosity_arg(&input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_get_verbosity_arg_info() {
        let expected: LoggingLevel = LoggingLevel::Info;
        let input = vec!["patch".to_string(), "--loglevel=info".to_string()];

        let output = get_verbosity_arg(&input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_get_verbosity_arg_warn() {
        let expected: LoggingLevel = LoggingLevel::Warn;
        let input = vec!["patch".to_string(), "--loglevel=warn".to_string()];

        let output = get_verbosity_arg(&input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_get_verbosity_arg_error() {
        let expected: LoggingLevel = LoggingLevel::Error;
        let input = vec!["patch".to_string(), "--loglevel=error".to_string()];

        let output = get_verbosity_arg(&input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_get_verbosity_arg_default() {
        let expected: LoggingLevel = LoggingLevel::Info;
        let input = vec!["patch".to_string()];

        let output = get_verbosity_arg(&input);

        assert!(expected == output)
    }

    #[test]
    #[should_panic]
    fn should_panic_get_verbosity_arg() {
        let input = vec!["patch".to_string(), "--loglevel=pippo".to_string()];
        let _ = get_verbosity_arg(&input);
    }
}