use crate::Rule;
use regex::Regex;

lazy_static! {
    pub static ref CLASS_NAMING_ISSUE: Rule<String, (bool, String)> = Rule {
        warning_id: 0x00,
        worker: |class_name| {
            let _ = Regex::new("([a-z][a-z0-9]+[A-Z])+[a-z]+").unwrap();
            let num_regex = Regex::new("[0-9]+").unwrap();
            let mut fixed: String;
            fixed = num_regex.replace_all(&class_name, "").to_string();
            fixed.get_mut(0..1).map(|s| {
                s.make_ascii_lowercase();
                &*s
            });
            (fixed == class_name, fixed)
        }
    };
    pub static ref VARIABLE_NAMING_ISSUE: Rule<String, (bool, String)> = Rule {
        warning_id: 0x01,
        worker: |variable_name| {
            let _ = Regex::new("\\b([a-z][a-z0-9]+[A-Z])+[a-z0-9]").unwrap();
            let num_regex = Regex::new("[0-9]+").unwrap();
            let mut fixed: String;
            fixed = num_regex.replace_all(&variable_name, "").to_string();
            fixed.get_mut(0..1).map(|s| {
                s.make_ascii_lowercase();
                &*s
            });
            (variable_name == fixed, fixed)
        }
    };
    pub static ref FUNCTION_NAMING_ISSUE: Rule<String, (bool, String)> = Rule {
        warning_id: 0x04,
        worker: |param_name| {
            let _ = Regex::new("\\b([a-z][a-z0-9]+[A-Z])+[a-z0-9]").unwrap();
            let num_regex = Regex::new("[0-9]+").unwrap();
            let mut fixed: String;
            fixed = num_regex.replace_all(&param_name, "").to_string();
            fixed.get_mut(0..1).map(|s| {
                s.make_ascii_lowercase();
                &*s
            });
            (param_name == fixed, fixed)
        }
    };
    pub static ref FUNCTION_PARAM_NAMING_ISSUE: Rule<String, (bool, String)> = Rule {
        warning_id: 0x02,
        worker: |param_name| {
            let _ = Regex::new("\\b([a-z][a-z0-9]+[A-Z])+[a-z0-9]").unwrap();
            let num_regex = Regex::new("[0-9]+").unwrap();
            let mut fixed: String;
            fixed = num_regex.replace_all(&param_name, "").to_string();
            fixed.get_mut(0..1).map(|s| {
                s.make_ascii_lowercase();
                &*s
            });
            (param_name == fixed, fixed)
        }
    };
}
