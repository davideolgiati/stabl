use crate::system::shell_cmd_facade::ShellCmdFacade;
use crate::commons::string::split_string_using_delimiter;
use std::collections::HashMap;

pub fn get_updates_list() -> Vec<String> {
        let output: String = ShellCmdFacade::get_updateinfo_output();

        if output.is_empty() {
                return Vec::new();
        }

        split_string_using_delimiter(output, "\n")
                .drain(1..)
                .collect()
}

pub fn get_updates_details(updates_list: &[String]) -> Vec<String> {
        let stdout: String = ShellCmdFacade::get_repoquery_output(updates_list);
        
        split_string_using_delimiter(stdout, "\n")
}

pub fn get_installed_details(updates_list: &[String]) -> HashMap<String, Vec<String>> {
        assert!(!updates_list.is_empty());

        let output: String = ShellCmdFacade::get_rpm_output_for_local_packages(updates_list);

        split_string_using_delimiter(output, "\n")
                .into_iter()
                .map(|line| split_string_using_delimiter(line, "|#|"))
                .map(|details| (details[0].clone(), Vec::from([details[1].clone(), details[2].clone()])))
                .collect::<HashMap<String, Vec<String>>>()

}