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

pub fn get_release_arg(args: &[String]) -> SemanticVersion {
        let valid_args = [
                "--patch".to_string(),
                "--repack".to_string(),
                "--minor".to_string(),
                "--major" .to_string()
        ];

        let release_args: Vec<String> = args
                .iter().filter(|&arg| valid_args.contains(arg)).cloned()
                .collect();

        if release_args.is_empty() {
                return SemanticVersion::Repack
        }

        convert_release(release_args.last().unwrap())
}

pub fn look_for_help(args: &[String]) {
        if args.contains(&"--help".to_string()){
                let help:&str = r"          
Help:
        --major         only major and lower release are considered valid
        --minor         only minor and lower release are considered valid
        --patch         only patch and lower release are considered valid
        --repack        only repack and lower release are considered valid
        
        --help          display this help
    
        ";
                println!("{}", help);
                std::process::exit(0)
        }
}