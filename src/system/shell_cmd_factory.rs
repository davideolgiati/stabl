use crate::commons::string::split_filter_and_deduplicate_string_list;

use super::{cmd_args_builder::CmdArgsBuilder, shell};

pub struct ShellCmdFactory;
impl ShellCmdFactory {
        pub fn build_new_updateinfo() -> impl Fn() -> String {
                || {
                        println!("[i] getting updates list from remote...");
                        
                        let cmd_args = CmdArgsBuilder::new()
                                .add_base_args(&[
                                        String::from("updateinfo"), 
                                        String::from("list"), 
                                        String::from("--updates")
                                ])
                                .toggle_quiet_flag()
                                .build();

                        shell::run_command_and_read_stdout("dnf", &cmd_args)
                }
        }

        pub fn build_new_repoquery() -> impl Fn(&[String]) -> String {
                |updates_list: &[String]| {
                        let updates: Vec<String> = split_filter_and_deduplicate_string_list(
                                updates_list, " ", 3
                        ).into_iter().collect();
                
                        println!("[i] getting details from repository for {} update ...", updates.len());
                
                        let cmd_args = CmdArgsBuilder::new()
                                .add_base_args(&[String::from("repoquery")])
                                .toggle_cached_flag()
                                .toggle_quiet_flag()
                                .set_query_format_for_update_pkgs()
                                .add_additional_args(&updates)
                                .build();
                
                        shell::run_command_and_read_stdout("dnf", &cmd_args)
                }
        }
}