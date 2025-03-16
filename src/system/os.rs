use crate::system::shell;

pub fn get_os_name() -> String {
        let output: String = shell::run_command_and_read_stdout("uname", &[String::from("-n")]);

        assert!(!output.is_empty());

        output
}