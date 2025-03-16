use crate::system::cmd_args_builder::CmdArgsBuilder;
use crate::system::shell;
use crate::commons::string::split_string_using_delimiter;
use crate::commons::string::split_filter_and_deduplicate_string_list;
use std::collections::HashMap;

pub fn get_updates_list() -> Vec<String> {
        println!("[i] getting updates list from remote...");

        let args: &[&str] = &["updateinfo", "list", "--updates", "--quiet"];
        let stdout: String = shell::run_command_and_read_stdout("dnf", args);

        if stdout.is_empty() {
                return Vec::new();
        }

        let output: Vec<String> = split_string_using_delimiter(stdout, "\n")
                .drain(1..)
                .collect();

        output
}

pub fn get_updates_details(updates_list: &[String]) -> Vec<String> {
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

        let cmd_args: Vec<&str> = cmd_args.iter().map(|s| s.as_str()).collect();
        let output: String = shell::run_command_and_read_stdout("dnf", &cmd_args);
        let updates_by_line: Vec<String> = split_string_using_delimiter(output, "\n");

        updates_by_line
}

pub fn get_installed_details(updates_list: &[String]) -> HashMap<String, Vec<String>> {
        assert!(!updates_list.is_empty());
        
        let installed: Vec<String> = split_filter_and_deduplicate_string_list(
                updates_list, "|#|", 0
        ).into_iter().collect();

        println!("[i] getting details for {} installed packages ...", installed.len());

        let cmd_args = CmdArgsBuilder::new()
                .toggle_quiet_flag()
                .set_query_format_for_installed_pkgs()
                .add_additional_args(&installed)
                .build();

        let cmd_args: Vec<&str> = cmd_args.iter().map(|s| s.as_str()).collect();
        let output: String = shell::run_command_and_read_stdout("rpm", &cmd_args);

        split_string_using_delimiter(output, "\n")
                .into_iter()
                .map(|line| split_string_using_delimiter(line, "|#|"))
                .map(|details| (details[0].clone(), Vec::from([details[1].clone(), details[2].clone()])))
                .collect::<HashMap<String, Vec<String>>>()

}