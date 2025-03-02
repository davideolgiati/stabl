use std::process::Command;
use std::process::Output;

pub fn run_command_and_read_stdout(command: &str, args: Vec<String>) -> String {
        assert!(!command.is_empty());
        assert!(!args.is_empty());
        assert!(!command.contains(" "));
        
        let mut cmd = Command::new(command);

        for arg in args {
                assert!(!arg.is_empty());
                assert!(!arg.contains(" "));

                cmd.arg(&arg);
        }

        let console_output: Output = cmd
                .output()
                .unwrap_or_else(|_| panic!("failed to run {}!", command));
        
        let stdout:String = String::from_utf8(console_output.stdout)
                .unwrap_or_else(|_| panic!("{} returned an unparsable sequence of bytes!", command));
        
        stdout
}