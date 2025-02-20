use crate::system::shell;

pub fn get_os_name() -> String {
        let args: [&str; 1] = ["-n"];
        let output: String = shell::run_command_and_read_stdout(&"uname", &args);
        return output;
}