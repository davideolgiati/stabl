use std::process::Command;
use std::process::Output;

#[inline]
pub fn run_command_and_read_stdout(command: &str, args: &[String]) -> String {
        assert!(!command.is_empty());
        assert!(!args.is_empty());
        assert!(!command.contains(" "));
        
        let console_output: Output = Command::new(command)
                .args(args)
                .output()
                .unwrap_or_else(|_| panic!("failed to run {}!", command));
        
        let stdout:String = String::from_utf8(console_output.stdout)
                .unwrap_or_else(|_| panic!("{} returned an unparsable sequence of bytes!", command));
        
        stdout
}