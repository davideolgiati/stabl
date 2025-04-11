use crate::model::semantic_version::SemanticVersion;

#[inline]
fn convert_release(arg: &str) -> SemanticVersion {
        match arg {
                "--patch"  => SemanticVersion::Patch,
                "--repack" => SemanticVersion::Repack,
                "--minor"  => SemanticVersion::Minor,
                "--major"  => SemanticVersion::Major,
                _ => panic!("Invalid release type")
        }
}

pub fn get_release_arg(args: &[String], default_bump: SemanticVersion) -> SemanticVersion {
        let valid_args: [&str; 4] = ["--patch", "--repack", "--minor", "--major"];

        let release_args: Vec<String> = args.iter()
                .filter(|arg| {
                        let current_args: &str = arg.as_str();
                        valid_args.contains(&current_args)
                })
                .cloned()
                .collect();

        if release_args.is_empty() {
                return default_bump
        }

        convert_release(release_args.last().unwrap())
}

pub fn look_for_help(args: &[String]) {
        if args.contains(&"--help".to_string()){
                let help:&str = r"          
Help:
        --major         only major and lower release are considered valid
        --minor         only minor and lower release are considered valid
        --patch         only patch and lower release are considered valid (default)
        --repack        only repack and lower release are considered valid
        
        --help          display this help
    
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