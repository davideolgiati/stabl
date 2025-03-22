use crate::system::shell_cmd_facade::ShellCmdFacade;
use crate::commons::string::split_string_using_delimiter;

pub fn get_updates_list() -> Vec<String> {
        let output: String = ShellCmdFacade::get_updateinfo_output();

        if output.is_empty() {
                return Vec::new();
        }

        split_string_using_delimiter(&output, "\n")
                .drain(1..)
                .map(String::from)
                .collect()
}

pub fn get_updates_details(updates_list: &[&str]) -> Vec<String> {
        let stdout: String = ShellCmdFacade::get_repoquery_output(updates_list);
        
        split_string_using_delimiter(&stdout, "\n")
                .into_iter()
                .map(String::from)
                .collect()
}

pub fn get_installed_details(updates_list: &[&str]) -> Vec<String> {
        assert!(!updates_list.is_empty());

        let output: String = ShellCmdFacade::get_rpm_output_for_local_packages(updates_list);

        split_string_using_delimiter(&output, "\n")
                .into_iter()
                .map(String::from)
                .collect()
}