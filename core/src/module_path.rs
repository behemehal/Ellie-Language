use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};

/// Path type
/// ## Types
/// Directory
/// File
#[derive(Clone)]
pub enum PathType {
    Directory,
    File,
}

/// Level of path
/// / root / anotherDir / aYetAnotherDir / aFile
/// ## Fields
/// * `rtype` - Type of level [`PathType`]
/// * `name` - Name of level
/// * `index` - Index of level
#[derive(Clone)]
pub struct PathLevel {
    pub rtype: PathType,
    pub name: String,
    pub index: u8,
}

/// Virtual module pathing manager
/// ## Fields
/// *This struct not meant to be structed use [`ModulePath::new`] instead*
/// * `main_path` - Main path of package
/// * `levels` - [`Vec`] of [`PathLevel`]
/// * `path_messenger` - A function that will be called when a path is required
#[derive(Clone)]
pub struct ModulePath<E> {
    pub main_path: String,
    pub levels: Vec<PathLevel>,
    pub path_messenger: E,
}

impl<E> ModulePath<E>
where
    E: FnMut(String, Vec<PathLevel>, String) -> bool + Clone + Sized, //Current Module, required path
{
    /// Create a new module path
    /// # Arguments
    /// * `main_path` - The main path of the module
    /// * `path_messenger` - A function that will be called when a path is required
    /// # Example
    /// ```
    /// use ellie_core::module_path::ModulePath;
    /// let mut path = ModulePath::new("/home/user/Desktop/ourDir/file.ei", |main_path, levels, query| {
    ///    //Module will be asking us for paths to check if they exist
    ///    if Path::new(&query).exists() {
    ///        true
    ///    } else {
    ///       false
    ///    }  
    /// });
    pub fn new(path: String, path_messenger: E) -> Self {
        ModulePath {
            main_path: path,
            levels: vec![],
            path_messenger,
        }
    }

    /// Parse given path and return a new 'real' path
    /// # Examples
    /// ```
    /// use ellie_core::module_path::ModulePath;
    /// let mut path = ModulePath::new(String::from("/home/user/Desktop/ourDir/file.ei"), |main_path, levels, query| {
    ///    if Path::new(&query).exists() {
    ///        true
    ///    } else {
    ///       false
    ///    }
    /// });
    /// //A existing file path
    /// let result = path.parse_path(String::from("./subDir/file.ei")).unwrap();
    /// assert_eq!(result, String::from("/home/user/Desktop/ourDir/subDir/file.ei"));
    /// //A rule breaker file path
    /// let result = path.parse_path(String::from("../aModule.ei")).unwrap_err();
    /// assert_eq!(result, 0); //Code 0: Given path is out of workspace directory
    /// //A non-exists file path
    /// let result = path.parse_path(String::from("./aNonExistent.ei")).unwrap_err();
    /// assert_eq!(result, 1); //Code 1: Given path is not exists
    pub fn parse(&mut self, path: &str) -> Result<String, i8> {
        let messenger_response =
            (self.path_messenger)(self.main_path.clone(), self.levels.clone(), path.to_owned());
        if !messenger_response {
            Err(0)
        } else {
            Err(1)
        }
    }
}
