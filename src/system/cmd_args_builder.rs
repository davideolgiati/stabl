const UPDATE_QUERYFORMAT: &str = "%{name}|#|%{version}|#|%{release}|#|%{full_nevra}|#|%{name}-%{version}-%{release}.%{arch}";
const INSTALLED_QUERYFORMAT: &str = "%{name}|#|%{version}|#|%{release}";

pub struct CmdArgsBuilder {
        _quiet: bool,
        _cached: bool,
        _query_format_args: String,
        _base_args: Vec<String>,
        _additional_args: Vec<String>
}

impl CmdArgsBuilder {
        pub fn new() -> CmdArgsBuilder {
                CmdArgsBuilder{
                        _quiet: false,
                        _cached: false,
                        _query_format_args: String::from(""),
                        _base_args: Vec::new(),
                        _additional_args: Vec::new()
                }
        }

        pub fn toggle_quiet_flag(&mut self) -> &mut CmdArgsBuilder {
                self._quiet = !self._quiet;
                self
        }

        pub fn toggle_cached_flag(&mut self) -> &mut CmdArgsBuilder {
                self._cached = !self._cached;
                self
        }

        pub fn add_base_arg(&mut self, base_arg : &str) -> &mut CmdArgsBuilder {
                self._base_args.push(base_arg.to_owned());
                self
        }

        pub fn add_additional_args(&mut self, additional_args : &[String]) -> &mut CmdArgsBuilder {
                for item in additional_args {
                        self._additional_args.push(item.clone());
                }
                self
        }

        pub fn set_query_format_for_update_pkgs(&mut self) -> &mut CmdArgsBuilder {
                self._query_format_args = String::from(
                        UPDATE_QUERYFORMAT
                );
                self
        }

        pub fn set_query_format_for_installed_pkgs(&mut self) -> &mut CmdArgsBuilder {
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