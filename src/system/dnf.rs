use crate::system::shell_cmd_facade::ShellCmdFacade;
use crate::commons::string::split_string_using_delimiter;

pub fn get_updates_list<'a>() -> Vec<&'a str> {
        let output: String = ShellCmdFacade::get_updateinfo_output();

        if output.is_empty() {
                return Vec::new();
        }

        let output = Box::leak(output.into_boxed_str());

        split_string_using_delimiter(output, "\n")
                .iter()
                .skip(1)
                .copied()
                .collect()
}

pub fn get_updates_details<'a>(updates_list: &[&str]) -> Vec<&'a str> {
        let stdout: String = ShellCmdFacade::get_repoquery_output(updates_list);

        let stdout = Box::leak(stdout.into_boxed_str());
        
        split_string_using_delimiter(stdout, "\n").to_vec()
}

pub fn get_installed_details<'a>(updates_list: &[&str]) -> Vec<&'a str> {
        assert!(!updates_list.is_empty());

        let output: String = ShellCmdFacade::get_rpm_output_for_local_packages(updates_list);

        let output = Box::leak(output.into_boxed_str());

        split_string_using_delimiter(output, "\n").to_vec()
}