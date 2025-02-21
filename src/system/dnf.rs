use crate::system::shell;

pub fn get_available_updates() -> Vec<String> {
        let args: [&str; 4] = ["updateinfo", "list", "--updates", "--quiet"];
        let output: String = shell::run_command_and_read_stdout(&"dnf", &args);
        let updates_by_line: Vec<String> = output.split("\n").map(str::to_string).collect();

        return (&updates_by_line[1..]).to_vec();
}