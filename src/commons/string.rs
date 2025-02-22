pub fn split_string_using_delimiter(string: String, delimiter: &str) -> Vec<String> {
        assert!(string != "");
        assert!(delimiter != "");
        
        return string
                .split(delimiter)
                .into_iter()
                .clone()
                .filter(|&str| *str != *"")
                .map(str::to_string)
                .collect();
}