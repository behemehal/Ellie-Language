use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

/// Level of path
/// / root / anotherDir / aYetAnotherDir / aFile
/// ## Fields
/// * `rtype` - Type of level [`PathType`]
/// * `name` - Name of level
/// * `index` - Index of level
#[derive(Clone, Debug)]
pub struct PathLevel {
    pub name: String,
    pub index: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Leveler {
    pub levels: Vec<PathLevel>,
}

impl Leveler {
    pub fn new(path: &str) -> Leveler {
        Leveler {
            levels: path
                .split("/")
                .enumerate()
                .map(|(index, path)| PathLevel {
                    name: path.to_string(),
                    index,
                })
                .collect::<Vec<PathLevel>>(),
        }
    }

    pub fn pop_one(&mut self) -> bool {
        self.levels.pop().is_some()
    }

    /// Converts leveler to string
    /// ## Examples
    /// ```
    /// let leveler = Leveler::levelize_path("/root/anotherDir/aFile.txt", false);
    /// assert_eq!(leveler.to_string(), "/root/anotherDir/aFile.txt");
    /// ```
    pub fn to_string(&self) -> String {
        self.levels
            .iter()
            .map(|level| level.name.clone())
            .collect::<Vec<_>>()
            .join("/")
    }

    /// Join a new path to existing one
    /// ## Arguments
    /// * `path` - Path to join
    /// * `is_dir` - Is path a directory
    /// ## Returns
    /// [`Ok(Leveler)`] if path is valid, [`Err(String)`] otherwise
    /// ## Error codes
    /// * `0` - Out of path bounds
    /// * `1` - Path is not a directory
    pub fn join(&mut self, path: &str) -> i8 {
        for command in path.split("/").collect::<Vec<&str>>() {
            if command.starts_with("..") {
                if !self.pop_one() {
                    return 1;
                }
            } else if command == "." {
                continue;
            } else {
                self.levels.push(PathLevel {
                    name: command.to_string(),
                    index: self.levels.len(),
                });
            }
        }
        return 0;
    }
}

pub fn parse_module_import(path: &str, identifier: &str) -> Result<String, u8> {
    let mut base = Leveler::new(path);
    if base.levels.len() == 1 {
        Err(1)
    } else if base.pop_one() {
        base.join(identifier);
        Ok(base.to_string())
    } else {
        unreachable!()
    }
}
