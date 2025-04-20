use crate::model::semantic_version::SemanticVersion;
use crate::system::logger;

#[inline]
fn convert_release(arg: &str) -> SemanticVersion {
        match arg {
                "patch"  => SemanticVersion::Patch,
                "repack" => SemanticVersion::Repack,
                "minor"  => SemanticVersion::Minor,
                "major"  => SemanticVersion::Major,
                _ => panic!("Invalid release type - {}", arg)
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
                _ => panic!("Invalid verbosity level - {}", parsed_arg)
        }
}

pub fn get_release_arg(release_arg: &str) -> SemanticVersion {
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
    use super::*;
    
    #[test]
    fn happy_path_convert_release_patch() {
        let expected: SemanticVersion = SemanticVersion::Patch;
        let input: &str = "--patch";

        let output = convert_release(input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_convert_release_repack() {
        let expected: SemanticVersion = SemanticVersion::Repack;
        let input: &str = "--repack";

        let output = convert_release(input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_convert_release_minor() {
        let expected: SemanticVersion = SemanticVersion::Minor;
        let input: &str = "--minor";

        let output = convert_release(input);

        assert!(expected == output)
    }

    #[test]
    fn happy_path_convert_release_major() {
        let expected: SemanticVersion = SemanticVersion::Major;
        let input: &str = "--major";

        let output = convert_release(input);

        assert!(expected == output)
    }

    #[test]
    #[should_panic]
    fn should_panic_convert_release() {
        let input: &str = "--bugfix";
        convert_release(input);
    }
}