pub fn split_string_using_delimiter(string: String, delimiter: &str) -> Vec<String> {
        assert!(!string.is_empty());
        assert!(!delimiter.is_empty());
        
        string
                .split(delimiter)
                .clone()
                .filter(|&str| *str != *"")
                .map(str::to_string)
                .collect()
}