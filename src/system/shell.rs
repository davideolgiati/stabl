use std::process::Command;
use std::process::Output;

pub fn run_command_and_read_stdout(command: &str, args: &[&str]) -> String {
        let system_details:&Output = &Command::new(&command)
                .args(&*args)
                .output()
                .expect("failed to get system details"); //TODO: errore specifico
        
        let stdout:String = String::from_utf8(system_details.stdout.clone())
                .expect("System details returned an unparsable UTF-8 string"); //TODO: errore specifico
        
        return stdout;
}