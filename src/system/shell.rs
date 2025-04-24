use std::process::Command;
use std::process::Output;

#[inline]
pub fn run_command_and_read_stdout(command: &str, args: &[String]) -> String {
        crate::debug!("run_command_and_read_stdout() IN");

        assert!(!command.is_empty());
        crate::trace!("command is not empty");

        assert!(!args.is_empty());
        crate::trace!("args are not empty");

        assert!(!command.contains(" "));
        crate::trace!("command does not contain any spaces");
        
        crate::trace!("running \"{}\" as subprocess", command);
        let console_output: Output = Command::new(command)
                .args(args)
                .output()
                .unwrap_or_else(|_| {
                        crate::error!("failed to run {}!", command);
                        std::process::exit(1);
                });
        crate::trace!("subprocess exited successfully");
        
        let stdout:String = String::from_utf8(console_output.stdout)
                .unwrap_or_else(|_| panic!("{} returned an unparsable sequence of bytes!", command));

        crate::debug!("run_command_and_read_stdout() OUT");
        stdout
}