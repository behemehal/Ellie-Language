use crate::Rule;
use regex::Regex;

lazy_static! {
    pub static ref CLASS_NAMING_ISSUE: Rule<String, (bool, String)> = Rule {
        warning_id: 0,
        worker: |class_name| {
            let regex = Regex::new("([A-Z][a-z0-9]+[A-Z])+[a-z]+").unwrap();
            let num_regex = Regex::new("[0-9]+").unwrap();
            let mut fixed: String;
            fixed = num_regex.replace_all(&class_name, "").to_string();
            fixed.get_mut(0..1).map(|s| {
                s.make_ascii_uppercase();
                &*s
            });
            (regex.is_match(&class_name), fixed)
        }
    };
    pub static ref VARIABLE_NAMING_ISSUE: Rule<String, (bool, String)> = Rule {
        warning_id: 1,
        worker: |variable_name| {
            let regex = Regex::new("\\b([a-z][a-z0-9]+[A-Z])+[a-z0-9]").unwrap();
            let num_regex = Regex::new("[0-9]+").unwrap();
            let mut fixed: String;
            fixed = num_regex.replace_all(&variable_name, "").to_string();
            fixed.get_mut(0..1).map(|s| {
                s.make_ascii_lowercase();
                &*s
            });
            (regex.is_match(&variable_name), fixed)
        }
    };
    pub static ref FUNCTION_PARAM_NAMING_ISSUE: Rule<String, (bool, String)> = Rule {
        warning_id: 0,
        worker: |param_name| {
            let regex = Regex::new("\\b([a-z][a-z0-9]+[A-Z])+[a-z0-9]").unwrap();
            let num_regex = Regex::new("[0-9]+").unwrap();
            let mut fixed: String;
            fixed = num_regex.replace_all(&param_name, "").to_string();
            fixed.get_mut(0..1).map(|s| {
                s.make_ascii_lowercase();
                &*s
            });
            (regex.is_match(&param_name), fixed)
        }
    };
}
