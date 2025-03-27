use crate::commons::string::split_filter_and_deduplicate_string_list;

use super::cmd_args_builder::CmdArgsBuilder;

type ShellCmdClosure = fn(&str, &[String]) -> String;

pub struct ShellCmdFacade;

impl ShellCmdFacade {
        pub fn get_updateinfo_output(_shell_cmd: ShellCmdClosure) -> String {
                println!("[i] getting updates list from remote...");

                let cmd_args = CmdArgsBuilder::new()
                        .add_base_arg("updateinfo")
                        .add_base_arg("list")
                        .add_base_arg("--updates")
                        .toggle_quiet_flag()
                        .build();

                assert!(!cmd_args.is_empty());

                _shell_cmd("dnf", &cmd_args)
        }

        pub fn get_repoquery_output(updates_list: &[&str], _shell_cmd: ShellCmdClosure) -> String {
                let updates: Vec<&str> = split_filter_and_deduplicate_string_list(
                        updates_list, " ", 3
                );
        
                assert!(!updates.is_empty());

                println!("[i] getting details from repository for {} update ...", updates.len());
        
                let cmd_args = CmdArgsBuilder::new()
                        .add_base_arg("repoquery")
                        .toggle_cached_flag()
                        .toggle_quiet_flag()
                        .set_query_format_for_update_pkgs()
                        .add_additional_args(&updates)
                        .build();

                assert!(!cmd_args.is_empty());
        
                _shell_cmd("dnf", &cmd_args)
        }

        pub fn get_rpm_output_for_local_packages(updates_list: &[&str], _shell_cmd: ShellCmdClosure) -> String {
                let installed: Vec<&str> = split_filter_and_deduplicate_string_list(
                        updates_list, "|#|", 0
                );
        
                assert!(!installed.is_empty());

                println!("[i] getting details for {} installed packages ...", installed.len());
        
                let cmd_args = CmdArgsBuilder::new()
                        .toggle_quiet_flag()
                        .set_query_format_for_installed_pkgs()
                        .add_additional_args(&installed)
                        .build();

                assert!(!cmd_args.is_empty());
        
                _shell_cmd("rpm", &cmd_args)
        }

}