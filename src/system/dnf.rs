use crate::system::shell;

pub fn get_available_updates() -> String {
        let args: [&str; 4] = ["updateinfo", "list", "--updates", "--json"];
        let output: String = shell::run_command_and_read_stdout(&"dnf", &args);
        return output;
}