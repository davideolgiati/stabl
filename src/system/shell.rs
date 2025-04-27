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

pub fn mock_shell_cmds(cmd: &str, args: &[String]) -> String {
        let updateinfo_output = concat!(
                "Name                   Type        Severity                                            Package              Issued\n",
                "FEDORA-2025-1a0c45a564 enhancement None                   vim-minimal-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07\n",
                "FEDORA-2025-1a0c45a564 enhancement None                           xxd-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07\n",
        ).to_string();
        let repoquery_output = concat!(
                "vim-minimal|#|9.1.1227|#|1.fc41|#|vim-minimal-2:9.1.1227-1.fc41.x86_64|#|vim-minimal-9.1.1227-1.fc41.x86_64\n",
                "xxd|#|9.1.1227|#|1.fc41|#|xxd-2:9.1.1227-1.fc41.x86_64|#|xxd-9.1.1227-1.fc41.x86_64"
        ).to_string();
        let rpm_output = concat!(
                "vim-minimal|#|9.1.1202|#|1.fc41\n",
                "xxd|#|9.1.1202|#|1.fc41"
        ).to_string();
    
        match cmd {
            "dnf" => {
                match args[0].as_str() {
                    "updateinfo" => updateinfo_output,
                    "makecache" => "done.".to_string(),
                    "repoquery" => repoquery_output,
                    _ => panic!("unknown branch!")
                }
            },
            "rpm" => rpm_output,
            _ => panic!("unknown branch!")
        }
    }