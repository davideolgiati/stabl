use std::process::Command;
use std::process::Output;

fn run_command_and_read_stdout(command: &str, args: &[&str]) -> String {
        let system_details:&Output = &Command::new(&command)
                .args(&*args)
                .output()
                .expect("failed to get system details");
        
        let stdout:String = String::from_utf8(system_details.stdout.clone())
                .expect("System details returned an unparsable UTF-8 string");
        
        return stdout;
}

pub fn get_system_details() -> String {
        let output: String = run_command_and_read_stdout(&"uname", &[&"-n"]);
        return output;
}