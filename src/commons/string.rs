use std::collections::HashSet;

#[inline]
pub fn split_string<'a>(string: &'a str, delimiter: &'a str) -> Vec<&'a str> {
        assert!(!string.is_empty());
        assert!(!delimiter.is_empty());
        assert!(string.contains(delimiter), "{}, {}", string, delimiter);

        let output: Vec<&str> = string
                .split(delimiter)
                .filter(|str| !str.is_empty())
                .collect();

        assert!(!output.is_empty());

        output
}

pub fn split_filter_and_deduplicate_string_list<'a>(list: &[&'a str], delimiter: &'a str, offset: usize) -> Vec<&'a str>{
    assert!(!list.is_empty());
    assert!(!delimiter.is_empty());

    list
        .iter()
        .map(|line: &&str| split_string(line, delimiter)[offset])
        .collect::<HashSet<&str>>()
        .iter()
        .cloned()
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn usecase_updatelist() {
        let test: &str = "FEDORA-2025-f14b0ee7be enhancement None                           firefox-131.0.2-1.fc41.x86_64 2025-03-17 01:37:24";
        let expected: Vec<&str> = vec!["FEDORA-2025-f14b0ee7be", "enhancement", "None", "firefox-131.0.2-1.fc41.x86_64", "2025-03-17", "01:37:24"];
        let result = split_string(test, " ");
        assert_eq!(result, expected);
    }

    #[test]
    fn usecase_repoquery() {
        let test: &str = "firefox|#|131.0.2|#|1.fc41|#|firefox-0:131.0.2-1.fc41.x86_64|#|firefox-131.0.2-1.fc41.x86_64";
        let expected: Vec<&str> = vec!["firefox","131.0.2","1.fc41","firefox-0:131.0.2-1.fc41.x86_64","firefox-131.0.2-1.fc41.x86_64"];
        let result = split_string(test, "|#|");
        assert_eq!(result, expected);
    }

    #[test]
    fn usecase_rpm() {
        let test: &str = "firefox|#|136.0.1|#|1.fc41";
        let expected: Vec<&str> = vec!["firefox","136.0.1","1.fc41"];
        let result = split_string(test, "|#|");
        assert_eq!(result, expected);
    }

    #[test]
    fn happy_path_multi_line_split() {
        let test: &str = "Hello\nWorld!";
        let expected: Vec<&str> = vec!["Hello", "World!"];
        let result = split_string(test, "\n");
        assert_eq!(result, expected);
    }

    #[test]
    fn happy_path_multi_line_split_with_empty_line() {
        let test: &str = "Hello\n\nWorld!";
        let expected: Vec<&str> = vec!["Hello", "World!"];
        let result = split_string(test, "\n");
        assert_eq!(result, expected);
    }
    
    #[test]
    fn happy_path_space_split() {
        let test: &str = "Hello World!";
        let expected: Vec<&str> = vec!["Hello", "World!"];
        let result = split_string(test, " ");
        assert_eq!(result, expected);
    }

    #[test]
    fn happy_path_space_split_with_empty_line() {
        let test: &str = "Hello  World!";
        let expected: Vec<&str> = vec!["Hello", "World!"];
        let result = split_string(test, " ");
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn panic_space_split_empty_value() {
        let test: &str = " ";
        split_string(test, " ");
    }

    #[test]
    #[should_panic]
    fn panic_no_string() {
        let test: &str  = "";
        split_string(test, "\n");
    }

    #[test]
    #[should_panic]
    fn panic_no_delimiter() {
        let test: &str  = "Hello World!";
        split_string(test, "");
    }

    #[test]
    #[should_panic]
    fn panic_delimiter_not_in_string() {
        let test: &str  = "Hello World!";
        split_string(test, "\n");
    }

    #[test]
    fn happy_path_split_filter_and_deduplicate_string_list_with_duplicate_line() {
        let test: Vec<&str> = vec![
            "test1|#|131.0.2|#|1.fc41",
            "test1|#|131.0.2|#|1.fc41",
            "test2|#|1.0.0|#|1.fc41",
        ];
        let expected: Vec<&str> = vec!["131.0.2","1.0.0"];
        let result = split_filter_and_deduplicate_string_list(&test, "|#|", 1);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn panic_split_filter_and_deduplicate_string_list_empty_list() {
        split_filter_and_deduplicate_string_list(&Vec::new(), " ", 0);
    }

    #[test]
    #[should_panic]
    fn panic_split_filter_and_deduplicate_string_list_empty_delimeiter() {
        split_filter_and_deduplicate_string_list(&["test"], "", 0);
    }
}