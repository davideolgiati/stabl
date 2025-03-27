use crate::system::shell_cmd_facade::ShellCmdFacade;
use crate::commons::string::split_string_using_delimiter;

type ShellCmdClosure = fn(&str, &[String]) -> String;

pub fn get_updates_list<'a>(_shell_cmd: ShellCmdClosure) -> Vec<&'a str> {
        let output: String = ShellCmdFacade::get_updateinfo_output(
                _shell_cmd
        );

        if output.is_empty() {
                return Vec::new();
        }

        assert!(!output.is_empty());

        let output = Box::leak(output.into_boxed_str());

        split_string_using_delimiter(output, "\n")
                .iter()
                .skip(1)
                .copied()
                .collect()
}

pub fn get_updates_details<'a>(updates_list: &[&str], _shell_cmd: ShellCmdClosure) -> Vec<&'a str> {
        let stdout: String = ShellCmdFacade::get_repoquery_output(
                updates_list, _shell_cmd
        );

        assert!(!stdout.is_empty());

        let stdout: &str = Box::leak(stdout.into_boxed_str());
        
        split_string_using_delimiter(stdout, "\n").to_vec()
}

pub fn get_installed_details<'a>(updates_list: &[&str], _shell_cmd: ShellCmdClosure) -> Vec<&'a str> {
        assert!(!updates_list.is_empty());

        let output: String = ShellCmdFacade::get_rpm_output_for_local_packages(
                updates_list, _shell_cmd
        );

        assert!(!output.is_empty());

        let output: &str = Box::leak(output.into_boxed_str());

        split_string_using_delimiter(output, "\n").to_vec()
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

    #[test]
    fn happy_path_get_updates_list() {
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
        let output = get_updates_list(GET_UPDATE_LIST_MOCK);
        assert_eq!(output.len(), 11);
        assert_eq!(output, expected);
    }
}