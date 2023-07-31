use alloc::{borrow::ToOwned, string::String, vec::Vec, format};
use core::fmt::{Display, Error, Formatter};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[cfg(feature = "compiler_utils")]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum TokenizerType {
    Raw,
    ClassParser,
    FunctionParser,
    HeaderParser,
}

#[cfg(feature = "compiler_utils")]
impl Default for TokenizerType {
    fn default() -> Self {
        TokenizerType::Raw
    }
}

#[cfg(feature = "compiler_utils")]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerOptions {
    pub path: String,
    pub functions: bool,
    pub break_on_error: bool,
    pub loops: bool,
    pub enums: bool,
    pub classes: bool,
    pub getters: bool,
    pub setters: bool,
    pub conditions: bool,
    pub global_variables: bool,
    pub line_ending: String,
    pub dynamics: bool,
    pub collectives: bool,
    pub variables: bool,
    pub import_std: bool,
    pub constants: bool,
    pub ignore_imports: bool,
    pub parser_type: TokenizerType,
    pub allow_import: bool,
}

#[cfg(feature = "compiler_utils")]
impl Default for TokenizerOptions {
    fn default() -> Self {
        TokenizerOptions {
            path: "".to_owned(),
            functions: true,
            break_on_error: false,
            loops: true,
            conditions: true,
            getters: true,
            setters: true,
            classes: true,
            enums: true,
            global_variables: true,
            line_ending: "\\r\\n".to_owned(),
            dynamics: true,
            import_std: true,
            collectives: true,
            ignore_imports: false,
            variables: true,
            constants: true,
            parser_type: TokenizerType::Raw,
            allow_import: true,
        }
    }
}

/// A struct that represents a position in a file.
/// (line, column)
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct CursorPosition(pub usize, pub usize);

impl core::fmt::Display for CursorPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl CursorPosition {
    pub fn is_bigger(&self, other: &CursorPosition) -> bool {
        self.0 > other.0 || (self.0 == other.0 && self.1 > other.1)
    }

    pub fn skip_char(&mut self, n: usize) -> CursorPosition {
        let mut clone = *self;
        clone.1 += n;
        clone
    }

    pub fn pop_char(&mut self, n: usize) -> CursorPosition {
        let mut clone = *self;
        if clone.1 != 0 {
            clone.1 -= n;
        }
        clone
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    pub fn increase_line(&mut self, n: usize) -> CursorPosition {
        let mut clone = *self;
        clone.0 += n;
        clone
    }
}

/// Cursor position
/// ## Fields
/// * `range_start` - Start of range [`CursorPosition`]
/// * `range_end` - End of range [`CursorPosition`]
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Cursor {
    pub range_start: CursorPosition,
    pub range_end: CursorPosition,
}

impl Cursor {
    /// Check range_start and range_end is zero by column and lines
    pub fn is_zero(&self) -> bool {
        self.range_start.is_zero() && self.range_end.is_zero()
    }

    /// Check current cursor bigger than given [`Cursor`]
    /// ## Arguments
    /// * `cursor` - [`Cursor`] to compare
    pub fn is_bigger(&self, than: Cursor) -> bool {
        if than.range_end.0 == self.range_end.0 {
            self.range_end.1 > than.range_end.1
        } else {
            than.range_end.0 <= self.range_end.0
        }
    }

    /// Create new [`Cursor`] range start and skip one column pos to define the end
    /// ## Arguments
    /// * `start` - Start of range [`CursorPosition`]
    pub fn build_with_skip_char(range_start: CursorPosition) -> Self {
        Cursor {
            range_start,
            range_end: range_start.clone().skip_char(1),
        }
    }

    /// Create new [`Cursor`]
    /// ## Arguments
    /// * `start` - Start of range [`CursorPosition`]
    pub fn build_from_cursor(range_start: CursorPosition) -> Self {
        Cursor {
            range_start,
            range_end: range_start,
        }
    }

    /// Gets [`Cursor`] range end and skip one char
    /// ## Arguments
    /// * `n` - Number of chars to skip
    /// ## Returns
    /// [`Cursor`] with new range end
    pub fn range_end_skip_char(&self, n: usize) -> Self {
        self.range_end.clone().skip_char(n);
        *self
    }

    /// Gets [`Cursor`] range start and skip one char
    /// ## Arguments
    /// * `n` - Number of chars to skip
    /// ## Returns
    /// [`Cursor`] with new range start and end
    pub fn range_start_skip_char(&self, n: usize) -> Self {
        self.range_start.clone().skip_char(n);
        *self
    }
}

/// Version
/// ## Fields
/// * `major` - Major version [`u8`]
/// * `minor` - Minor version [`u8`]
/// * `bug` - Bug version [`u8`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub pre_release: Option<String>,
    pub build_metadata: Option<String>,
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        //Ignore bug
        self.minor == other.minor && self.major == other.major
    }
}

impl Version {
    /// Create new [`Version`] from given [`String`]
    /// ## Arguments
    /// * `version` - [`String`] to parse
    pub fn build_from_string(input: &String) -> Version {
        let semver_regex = Regex::new(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
        let caps = semver_regex.captures(&input).unwrap();

        Version {
            major: caps
                .name("major")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            minor: caps
                .name("minor")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            patch: caps
                .name("patch")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            pre_release: caps.name("prerelease").map(|x| x.as_str().to_owned()),
            build_metadata: caps.name("buildmetadata").map(|x| x.as_str().to_owned()),
        }
    }

    /// Create new [`Version`] from given [`String`] with checks
    /// ## Arguments
    /// * `input` - [`String`] to parse
    /// ## Return
    /// [`Result`] - If versionb is valid [`Ok(Version)`] otherwise [`Err(u8)`]-
    pub fn build_from_string_checked(input: &String) -> Result<Version, u8> {
        let semver_regex = Regex::new(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
        match semver_regex.captures(&input) {
            Some(caps) => {
                let major = caps
                    .name("major")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap_or(0);
                let minor = caps
                    .name("minor")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap_or(0);
                let patch = caps
                    .name("patch")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap_or(0);

                let pre_release = caps.name("prerelease").map(|x| x.as_str().to_owned());
                let build_metadata = caps.name("buildmetadata").map(|x| x.as_str().to_owned());

                if major == 0 && minor == 0 && patch == 0 {
                    Err(1)
                } else {
                    Ok(Version {
                        minor,
                        major,
                        patch,
                        pre_release,
                        build_metadata,
                    })
                }
            }
            None => Err(1),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}.{}.{}{}{}",
            self.major,
            self.minor,
            self.patch,
            if let Some(ref pre_release) = self.pre_release {
                format!("-{}", pre_release)
            } else {
                "".to_owned()
            },
            if let Some(ref build_metadata) = self.build_metadata {
                format!("+{}", build_metadata)
            } else {
                "".to_owned()
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum PlatformArchitecture {
    B16,
    B32,
    B64,
}

impl Display for PlatformArchitecture {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            PlatformArchitecture::B16 => write!(f, "b16"),
            PlatformArchitecture::B32 => write!(f, "b32"),
            PlatformArchitecture::B64 => write!(f, "b64"),
        }
    }
}

impl PlatformArchitecture {
    pub fn is_16(&self) -> bool {
        match self {
            PlatformArchitecture::B16 => true,
            _ => false,
        }
    }

    pub fn is_32(&self) -> bool {
        match self {
            PlatformArchitecture::B32 => true,
            _ => false,
        }
    }

    pub fn get_code(&self) -> u8 {
        match self {
            PlatformArchitecture::B16 => 16,
            PlatformArchitecture::B32 => 32,
            PlatformArchitecture::B64 => 64,
        }
    }

    pub fn type_id_size(&self) -> u8 {
        match self {
            PlatformArchitecture::B16 => 3,
            PlatformArchitecture::B32 => 5,
            PlatformArchitecture::B64 => 9,
        }
    }

    pub fn usize_len(&self) -> u8 {
        match self {
            PlatformArchitecture::B16 => 2,
            PlatformArchitecture::B32 => 4,
            PlatformArchitecture::B64 => 8,
        }
    }

    pub fn from_byte(byte: u8) -> Option<PlatformArchitecture> {
        match byte {
            16 => Some(PlatformArchitecture::B16),
            32 => Some(PlatformArchitecture::B32),
            64 => Some(PlatformArchitecture::B64),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DebugHeaderType {
    Variable,
    SetterCall,
    GetterCall,
    Class,
    Parameter,
    Function,
    NativeFunction,
    Condition,
}

#[derive(Clone, Debug)]
pub struct DebugHeader {
    /// Element Type
    pub rtype: DebugHeaderType,
    /// Element's hash
    pub hash: usize,
    /// Module Name
    pub module_name: String,
    /// Module Hash
    pub module_hash: usize,
    /// Element Name
    pub name: String,
    /// Instruction start -> end,
    pub start_end: (usize, usize),
    /// Code pos
    pub pos: Cursor,
}

#[derive(Debug, Clone)]
pub struct ModuleMap {
    pub module_name: String,
    pub module_hash: usize,
    pub module_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NativeCallTrace {
    pub module_name: String,
    pub function_hash: usize,
    pub function_name: String,
}

#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub module_map: Vec<ModuleMap>,
    pub debug_headers: Vec<DebugHeader>,
}
