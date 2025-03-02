pub fn split_string_using_delimiter(string: String, delimiter: &str) -> Vec<String> {
        assert!(!string.is_empty());
        assert!(!delimiter.is_empty());
        assert!(string.contains(delimiter));
        
        let output: Vec<String> = string
                .split(delimiter)
                .clone()
                .filter(|&str| *str != *"")
                .map(str::to_string)
                .collect();

        assert!(!output.is_empty());
        assert!(output.len() >= 2);

        output
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn happy_path() {
        let test: String = "Hello World!".to_string();
        let expected: Vec<String> = vec!["Hello".to_string(), "World!".to_string()];
        let result = split_string_using_delimiter(test, " ");
        assert_eq!(result, expected);
    }
}