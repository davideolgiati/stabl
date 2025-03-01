use crate::system::shell;
use crate::commons::string::split_string_using_delimiter;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn get_updates_list() -> Vec<String> {
        println!("[i] getting updates list from remote...");

        let args: [&str; 4] = ["updateinfo", "list", "--updates", "--quiet"];
        let output: String = shell::run_command_and_read_stdout("dnf", &args);
        let updates_by_line: Vec<String> = split_string_using_delimiter(output, "\n");

        updates_by_line[1..].to_vec()
}

pub fn get_updates_details(updates_list: &[String]) -> Vec<String> {
        println!("[i] getting update details from repository...");
    
        let updates: Vec<String> = updates_list
                .iter()
                .cloned()
                .map(|line: String| split_string_using_delimiter(line, " "))
                .map(|tokens: Vec<String>| tokens[3].clone())
                .collect::<Vec<String>>();

        let mut args:Vec<&str> = Vec::from([
                "repoquery", "-C", 
                "--quiet", 
                "--queryformat=%{name}|#|%{version}|#|%{release}|#|%{full_nevra}|#|%{name}-%{version}-%{release}.%{arch}\\n",
        ]);

        for signature in &updates {
                args.push(signature.as_str());
        }
        
        let output: String = shell::run_command_and_read_stdout("dnf", &args);
        let updates_by_line: Vec<String> = split_string_using_delimiter(output, "\n");

        updates_by_line
}

pub fn get_installed_details(updates_list: &[String]) -> HashMap<String, Vec<String>> {
        println!("[i] getting installed packages details...");
        
        let installed = updates_list
                .iter()
                .cloned()
                .map(|line: String| split_string_using_delimiter(line, "|#|"))
                .map(|items| items[0].clone())
                .collect::<HashSet<String>>();
                

        let mut args:Vec<&str> = Vec::from([
                "-q", 
                "--queryformat=%{name}|#|%{version}|#|%{release}\\n"
        ]);
        
        for package in &installed {
                args.push(package.as_str());
        }

        let output: String = shell::run_command_and_read_stdout("rpm", &args);

        split_string_using_delimiter(output, "\n")
                .into_iter()
                .map(|line| split_string_using_delimiter(line, "|#|"))
                .map(|details| (details[0].clone(), Vec::from([details[1].clone(), details[2].clone()])))
                .collect::<HashMap<String, Vec<String>>>()

}