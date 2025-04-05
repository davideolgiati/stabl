use crate::commons::string::{split_filter_and_deduplicate_string_list, split_string};

type ShellCmdClosure = fn(&str, &[String]) -> String;

const UPDATE_QUERYFORMAT: &str = "%{name}|#|%{version}|#|%{release}|#|%{full_nevra}|#|%{name}-%{version}-%{release}.%{arch}";
const INSTALLED_QUERYFORMAT: &str = "%{name}|#|%{version}|#|%{release}";

#[derive(Default)]
struct ArgsBuilder {
        _quiet: bool,
        _cached: bool,
        _query_format_args: String,
        _base_args: Vec<String>,
        _additional_args: Vec<String>
}

impl ArgsBuilder {
        pub fn new() -> ArgsBuilder {
                Default::default()
        }

        pub fn toggle_quiet_flag(&mut self) -> &mut ArgsBuilder {
                self._quiet = !self._quiet;
                self
        }

        pub fn toggle_cached_flag(&mut self) -> &mut ArgsBuilder {
                self._cached = !self._cached;
                self
        }

        pub fn add_base_arg(&mut self, base_arg : &str) -> &mut ArgsBuilder {
                self._base_args.push(base_arg.to_owned());
                self
        }

        pub fn add_additional_args(&mut self, additional_args : &[&str]) -> &mut ArgsBuilder {
                for item in additional_args {
                        self._additional_args.push(item.to_string());
                }
                self
        }

        pub fn set_query_format_for_update_pkgs(&mut self) -> &mut ArgsBuilder {
                self._query_format_args = String::from(
                        UPDATE_QUERYFORMAT
                );
                self
        }

        pub fn set_query_format_for_installed_pkgs(&mut self) -> &mut ArgsBuilder {
                self._query_format_args = String::from(
                        INSTALLED_QUERYFORMAT
                );
                self
        }

        pub fn build(&mut self) -> Vec<String> {
                let mut output: Vec<String> = Vec::new();

                output.append(&mut self._base_args);

                if self._quiet {
                        output.push("-q".to_owned());
                }

                if self._cached {
                        output.push("-C".to_owned());
                }

                if !self._query_format_args.is_empty() {
                        output.push(format!("--queryformat={}\\n", self._query_format_args));
                }

                output.append(&mut self._additional_args);

                output
        }
}


pub fn get_updateinfo_output<'a>(_shell_cmd: ShellCmdClosure) -> Vec<&'a str> {
        println!("[i] getting updates list from remote...");

        let cmd_args = ArgsBuilder::new()
                .add_base_arg("updateinfo")
                .add_base_arg("list")
                .add_base_arg("--updates")
                .toggle_quiet_flag()
                .build();
        assert!(!cmd_args.is_empty());

        let output = _shell_cmd("dnf", &cmd_args);

        if output.is_empty() {
                return Vec::new();
        }
        assert!(!output.is_empty());

        let output = Box::leak(output.into_boxed_str());

        split_string(output, "\n")
                .iter()
                .skip(1)
                .copied()
                .collect()
}

pub fn get_repoquery_output<'a>(updates_list: &[&str], _shell_cmd: ShellCmdClosure) -> Vec<&'a str> {
        assert!(!updates_list.is_empty());
        
        let updates: Vec<&str> = split_filter_and_deduplicate_string_list(
                updates_list, " ", 3
        );
        assert!(!updates.is_empty());

        println!("[i] getting details from repository for {} update ...", updates.len());

        let cmd_args = ArgsBuilder::new()
                .add_base_arg("repoquery")
                .toggle_cached_flag()
                .toggle_quiet_flag()
                .set_query_format_for_update_pkgs()
                .add_additional_args(&updates)
                .build();
        assert!(!cmd_args.is_empty());

        let output = _shell_cmd("dnf", &cmd_args);

        if output.is_empty() {
                return Vec::new();
        }

        assert!(!output.is_empty());

        let stdout: &str = Box::leak(output.into_boxed_str());
        
        split_string(stdout, "\n").to_vec()
}

pub fn get_rpm_output_for_local_packages<'a>(updates_list: &[&str], _shell_cmd: ShellCmdClosure) -> Vec<&'a str> {
        assert!(!updates_list.is_empty());

        let installed: Vec<&str> = split_filter_and_deduplicate_string_list(
                updates_list, "|#|", 0
        );
        assert!(!installed.is_empty());

        println!("[i] getting details for {} installed packages ...", installed.len());

        let cmd_args = ArgsBuilder::new()
                .toggle_quiet_flag()
                .set_query_format_for_installed_pkgs()
                .add_additional_args(&installed)
                .build();
        assert!(!cmd_args.is_empty());

        let output = _shell_cmd("rpm", &cmd_args);

        if output.is_empty() {
                return Vec::new();
        }

        assert!(!output.is_empty());

        let output: &str = Box::leak(output.into_boxed_str());
        split_string(output, "\n").to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    static GET_UPDATE_LIST_MOCK: ShellCmdClosure = |_a, _b| concat!(
    "Name                   Type        Severity                                            Package              Issued\n",
    "FEDORA-2025-0738949695 unspecified None               python3-incremental-24.7.2-1.fc41.noarch 2025-03-18 02:17:53\n",
    "FEDORA-2025-1a0c45a564 enhancement None                      vim-data-2:9.1.1227-1.fc41.noarch 2025-03-23 01:13:07\n",
    "FEDORA-2025-1a0c45a564 enhancement None                   vim-minimal-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07\n",
    "FEDORA-2025-1a0c45a564 enhancement None                           xxd-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07\n",
    "FEDORA-2025-227a3afc1f enhancement None                             dpkg-1.22.15-1.fc41.x86_64 2025-03-21 01:13:34\n",
    "FEDORA-2025-227a3afc1f enhancement None                         dpkg-dev-1.22.15-1.fc41.noarch 2025-03-21 01:13:34\n",
    "FEDORA-2025-227a3afc1f enhancement None                        dpkg-perl-1.22.15-1.fc41.noarch 2025-03-21 01:13:34\n",
    "FEDORA-2025-4cd6805b63 enhancement None                        hwloc-libs-2.12.0-1.fc41.x86_64 2025-03-20 04:35:20\n",
    "FEDORA-2025-5d959bdf1d enhancement None                         libfprint-1.94.9-1.fc41.x86_64 2025-03-22 04:09:56\n",
    "FEDORA-2025-68a042226c enhancement None              container-selinux-4:2.236.0-1.fc41.noarch 2025-03-20 04:35:20\n",
    "FEDORA-2025-7755eec1cb unspecified None                  python3-regex-2024.11.6-1.fc41.x86_64 2025-03-12 02:01:22")
    .to_string();

    static GET_EMPTY_LIST_MOCK: ShellCmdClosure = |_a, _b| concat!("").to_string();

    #[test]
    fn happy_path_get_updateinfo_output() {
        let expected = vec![
        "FEDORA-2025-0738949695 unspecified None               python3-incremental-24.7.2-1.fc41.noarch 2025-03-18 02:17:53",
        "FEDORA-2025-1a0c45a564 enhancement None                      vim-data-2:9.1.1227-1.fc41.noarch 2025-03-23 01:13:07",
        "FEDORA-2025-1a0c45a564 enhancement None                   vim-minimal-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07",
        "FEDORA-2025-1a0c45a564 enhancement None                           xxd-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07",
        "FEDORA-2025-227a3afc1f enhancement None                             dpkg-1.22.15-1.fc41.x86_64 2025-03-21 01:13:34",
        "FEDORA-2025-227a3afc1f enhancement None                         dpkg-dev-1.22.15-1.fc41.noarch 2025-03-21 01:13:34",
        "FEDORA-2025-227a3afc1f enhancement None                        dpkg-perl-1.22.15-1.fc41.noarch 2025-03-21 01:13:34",
        "FEDORA-2025-4cd6805b63 enhancement None                        hwloc-libs-2.12.0-1.fc41.x86_64 2025-03-20 04:35:20",
        "FEDORA-2025-5d959bdf1d enhancement None                         libfprint-1.94.9-1.fc41.x86_64 2025-03-22 04:09:56",
        "FEDORA-2025-68a042226c enhancement None              container-selinux-4:2.236.0-1.fc41.noarch 2025-03-20 04:35:20",
        "FEDORA-2025-7755eec1cb unspecified None                  python3-regex-2024.11.6-1.fc41.x86_64 2025-03-12 02:01:22"
        ];
        let output = get_updateinfo_output(GET_UPDATE_LIST_MOCK);
        assert_eq!(output.len(), 11);
        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_get_updateinfo_empty_output() {
        let expected: Vec<&str> = Vec::new();
        let output: Vec<&str> = get_updateinfo_output(GET_EMPTY_LIST_MOCK);
        assert_eq!(output.len(), 0);
        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_get_repoquery_output_empty_output() {
        let expected: Vec<&str> = Vec::new();
        let output: Vec<&str> = get_repoquery_output(&["FEDORA-2025-7755eec1cb unspecified None                  python3-regex-2024.11.6-1.fc41.x86_64 2025-03-12 02:01:22"], GET_EMPTY_LIST_MOCK);
        assert_eq!(output.len(), 0);
        assert_eq!(output, expected);
    }

    #[test]
    fn happy_path_get_rpm_output_for_local_packages_empty_output() {
        let expected: Vec<&str> = Vec::new();
        let output: Vec<&str> = get_rpm_output_for_local_packages(&["\"firefox\"|#|\"131.0.2\"|#|\"1.fc41\"|#|\"firefox-0:131.0.2-1.fc41.x86_64\"|#|\"firefox-131.0.2-1.fc41.x86_64\""],GET_EMPTY_LIST_MOCK);
        assert_eq!(output.len(), 0);
        assert_eq!(output, expected);
    }

    #[test]
    #[should_panic]
    fn panic_get_repoquery_output_empty_input() {
        let _: Vec<&str> = get_repoquery_output(&[], GET_EMPTY_LIST_MOCK);
    }

    #[test]
    #[should_panic]
    fn panic_get_rpm_output_for_local_packages_empty_input() {
        let _: Vec<&str> = get_rpm_output_for_local_packages(&[],GET_EMPTY_LIST_MOCK);
    }
}