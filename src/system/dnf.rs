use crate::system::shell;
use crate::commons::string::split_string_using_delimiter;

pub fn get_available_updates() -> Vec<String> {
        let args: [&str; 4] = ["updateinfo", "list", "--updates", "--quiet"];
        let output: String = shell::run_command_and_read_stdout("dnf", &args);
        let updates_by_line: Vec<String> = split_string_using_delimiter(output, "\n");

        updates_by_line[1..].to_vec()
}

pub fn get_updates_details(signatures: Vec<String>) -> Vec<String> {
        let updates: Vec<&str> = signatures.iter().map(|x| &**x).collect();
        let args:Vec<&str> = Vec::from([
                "repoquery", "-C", 
                "--quiet", 
                "--queryformat=%{name}|#|%{version}|#|%{release}|#|%{arch}|#|%{full_nevra}|#|%{name}-%{version}-%{release}.%{arch}\\n",
        ]).into_iter().chain(updates).collect();
        
        let output: String = shell::run_command_and_read_stdout("dnf", &args);
        let updates_by_line: Vec<String> = split_string_using_delimiter(output, "\n");

        updates_by_line
}

pub fn get_installed_details(package_name: String) -> String {
        let args:Vec<&str> = Vec::from([
                "-q", 
                "--queryformat=%{name}|#|%{version}|#|%{release}\\n",
                package_name.as_str()
        ]);
        
        let output: String = shell::run_command_and_read_stdout("rpm", &args);
        let updates_by_line: Vec<String> = split_string_using_delimiter(output, "\n");

        updates_by_line[0].clone()
}