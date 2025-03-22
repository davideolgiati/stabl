use std::collections::HashSet;

#[inline]
pub fn split_string_using_delimiter<'a>(string: &'a str, delimiter: &'a str) -> Vec<&'a str> {
        assert!(!string.is_empty());
        assert!(!delimiter.is_empty());
        assert!(string.contains(delimiter));

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
        .map(|line: &&str| split_string_using_delimiter(line, delimiter)[offset])
        .collect::<HashSet<&str>>()
        .iter()
        .cloned()
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn happy_path_1() {
        let test: String = "Hello World!".to_string();
        let expected: Vec<String> = vec!["Hello".to_string(), "World!".to_string()];
        let result = split_string_using_delimiter(&test, " ");
        assert_eq!(result, expected);
    }
    
    #[test]
    fn happy_path_2() {
        let test: String = "one,two,three,four".to_string();
        let expected: Vec<String> = vec![
            "one".to_string(), "two".to_string(),
            "three".to_string(), "four".to_string()
        ];
        let result = split_string_using_delimiter(&test, ",");
        assert_eq!(result, expected);
    }
        
    #[test]
    fn happy_path_3() {
        let test: String = "Hello--World!".to_string();
        let expected: Vec<String> = vec!["Hello".to_string(), "World!".to_string()];
        let result = split_string_using_delimiter(&test, "--");
        assert_eq!(result, expected);
    }
        
    #[test]
    fn happy_path_4() {
        let test: String = "one,,two".to_string();
        let expected: Vec<String> = vec![
            "one".to_string(), "two".to_string()
        ];
        let result = split_string_using_delimiter(&test, ",");
        assert_eq!(result, expected);
    }
        
    #[test]
    fn happy_path_5() {
        let test: String = ",one,two,".to_string();
        let expected: Vec<String> = vec![
            "one".to_string(), "two".to_string()
        ];
        let result = split_string_using_delimiter(&test, ",");
        assert_eq!(result, expected);
    }
        
    #[test]
    #[should_panic]
    fn empty_string() {
        let test: String = "".to_string();
        split_string_using_delimiter(&test, ",");
    }

    #[test]
    #[should_panic]
    fn no_real_values_string() {
        let test: String = ",,,,".to_string();
        split_string_using_delimiter(&test, ",");
    }

    #[test]
    #[should_panic]
    fn no_delimiter_string() {
        let test: String = ",,,,".to_string();
        split_string_using_delimiter(&test, "");
    }

    #[test]
    #[should_panic]
    fn delimiter_not_in_string() {
        let test: String = ",,,,".to_string();
        split_string_using_delimiter(&test, "");
    }
}