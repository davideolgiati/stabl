use crate::system::shell;

pub fn get_os_name() -> String {
        let args: Vec<String> = vec!["-n".to_string()];
        let output: String = shell::run_command_and_read_stdout("uname", args);

        assert!(!output.is_empty());

        output
}