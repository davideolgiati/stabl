use crate::system::shell;
use crate::commons::string::split_string_using_delimiter;
use std::collections::HashSet;
use std::collections::HashMap;

const UPDATE_QUERYFORMAT: &str = "%{name}|#|%{version}|#|%{release}|#|%{full_nevra}|#|%{name}-%{version}-%{release}.%{arch}";
const INSTALLED_QUERYFORMAT: &str = "%{name}|#|%{version}|#|%{release}";

pub fn get_updates_list() -> Vec<String> {
        println!("[i] getting updates list from remote...");

        let args: [&str; 4] = ["updateinfo", "list", "--updates", "--quiet"];
        let output: String = shell::run_command_and_read_stdout("dnf", &args);

        if output.is_empty() {
                return Vec::new();
        }

        let mut updates_by_line: Vec<String> = split_string_using_delimiter(output, "\n");
        let output: Vec<String> = updates_by_line.drain(1..).collect();

        output
}

pub fn get_updates_details(updates_list: &[String]) -> Vec<String> {
        
        let updates: HashSet<String> = updates_list.iter().cloned()
                .map(|line: String| split_string_using_delimiter(line, " "))
                .map(|items: Vec<String>| items[3].clone())
                .collect::<HashSet<String>>();

        println!("[i] getting details from repository for {} update ...", updates.len());

        let query_format: String = format!("--queryformat={}\\n", UPDATE_QUERYFORMAT);
        let mut args:Vec<&str> = Vec::from(["repoquery", "-C", "--quiet", &query_format]);

        for signature in &updates {
                assert!(!signature.is_empty());
                assert!(!signature.contains(" "));

                args.push(signature.as_str());
        }
        
        let output: String = shell::run_command_and_read_stdout("dnf", &args);
        let updates_by_line: Vec<String> = split_string_using_delimiter(output, "\n");

        updates_by_line
}

pub fn get_installed_details(updates_list: &[String]) -> HashMap<String, Vec<String>> {
        assert!(!updates_list.is_empty());
        
        let installed = updates_list.iter().cloned()
                .map(|line: String| split_string_using_delimiter(line, "|#|"))
                .map(|items: Vec<String>| items[0].clone())
                .collect::<HashSet<String>>();

        println!("[i] getting details for {} installed packages ...", installed.len());

        let query_format: String = format!("--queryformat={}\\n", INSTALLED_QUERYFORMAT);
        let mut args:Vec<&str> = Vec::from(["-q", &query_format]);
        
        for package in &installed {
                assert!(!package.is_empty());
                assert!(!package.contains(" "));

                args.push(package.as_str());
        }

        let output: String = shell::run_command_and_read_stdout("rpm", &args);

        split_string_using_delimiter(output, "\n")
                .into_iter()
                .map(|line| split_string_using_delimiter(line, "|#|"))
                .map(|details| (details[0].clone(), Vec::from([details[1].clone(), details[2].clone()])))
                .collect::<HashMap<String, Vec<String>>>()

}