use crate::commons::string::split_filter_and_deduplicate_string_list;

use super::{cmd_args_builder::CmdArgsBuilder, shell};

pub struct ShellCmdFacade;
impl ShellCmdFacade {
        pub fn get_updateinfo_output() -> String {
                println!("[i] getting updates list from remote...");

                let cmd_args = CmdArgsBuilder::new()
                        .add_base_arg("updateinfo")
                        .add_base_arg("list")
                        .add_base_arg("--updates")
                        .toggle_quiet_flag()
                        .build();

                shell::run_command_and_read_stdout("dnf", &cmd_args)
        }

        pub fn get_repoquery_output(updates_list: &[String]) -> String {
                let updates: Vec<String> = split_filter_and_deduplicate_string_list(
                        updates_list, " ", 3
                ).into_iter().collect();
        
                println!("[i] getting details from repository for {} update ...", updates.len());
        
                let cmd_args = CmdArgsBuilder::new()
                        .add_base_arg("repoquery")
                        .toggle_cached_flag()
                        .toggle_quiet_flag()
                        .set_query_format_for_update_pkgs()
                        .add_additional_args(&updates)
                        .build();
        
                shell::run_command_and_read_stdout("dnf", &cmd_args)
        }

        pub fn get_rpm_output_for_local_packages(updates_list: &[String]) -> String {
                let installed: Vec<String> = split_filter_and_deduplicate_string_list(
                        updates_list, "|#|", 0
                ).into_iter().collect();
        
                println!("[i] getting details for {} installed packages ...", installed.len());
        
                let cmd_args = CmdArgsBuilder::new()
                        .toggle_quiet_flag()
                        .set_query_format_for_installed_pkgs()
                        .add_additional_args(&installed)
                        .build();
        
                shell::run_command_and_read_stdout("rpm", &cmd_args)
        }

}