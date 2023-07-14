use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::fmt::{Display, Error, Formatter};
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
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CursorPosition(pub usize, pub usize);

impl core::fmt::Display for CursorPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition(0, 0)
    }
}

impl CursorPosition {
    pub fn is_bigger(&self, other: &CursorPosition) -> bool {
        self.0 > other.0 || (self.0 == other.0 && self.1 > other.1)
    }

    pub fn skip_char(&mut self, n: usize) -> CursorPosition {
        let mut clone = self.clone();
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
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
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
        } else if than.range_end.0 > self.range_end.0 {
            return false;
        } else {
            return true;
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
        self.clone()
    }

    /// Gets [`Cursor`] range start and skip one char
    /// ## Arguments
    /// * `n` - Number of chars to skip
    /// ## Returns
    /// [`Cursor`] with new range start and end
    pub fn range_start_skip_char(&self, n: usize) -> Self {
        self.range_start.clone().skip_char(n);
        self.clone()
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            range_start: CursorPosition::default(),
            range_end: CursorPosition::default(),
        }
    }
}

/// Version
/// ## Fields
/// * `major` - Major version [`u8`]
/// * `minor` - Minor version [`u8`]
/// * `bug` - Bug version [`u8`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub bug: u8,
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
    pub fn build_from_string(input: String) -> Version {
        Version {
            minor: input.split(".").collect::<Vec<_>>()[0]
                .parse::<u8>()
                .unwrap_or_else(|_| panic!("Given 'minor', is not a number")),
            major: input.split(".").collect::<Vec<_>>()[1]
                .parse::<u8>()
                .unwrap_or_else(|_| panic!("Given 'major', is not a number")),
            bug: input.split(".").collect::<Vec<_>>()[2]
                .parse::<u8>()
                .unwrap_or_else(|_| panic!("Given 'bug', is not a number")),
        }
    }

    /// Create new [`Version`] from given [`String`] with checks
    /// ## Arguments
    /// * `input` - [`String`] to parse
    /// ## Return
    /// [`Result`] - If versionb is valid [`Ok(Version)`] otherwise [`Err(u8)`]-
    pub fn build_from_string_checked(input: String) -> Result<Version, u8> {
        if input.split(".").collect::<Vec<_>>().len() == 3 {
            let major = input.split(".").collect::<Vec<_>>()[0]
                .parse::<u8>()
                .unwrap_or(0);
            let minor = input.split(".").collect::<Vec<_>>()[1]
                .parse::<u8>()
                .unwrap_or(0);

            let bug = input.split(".").collect::<Vec<_>>()[2]
                .parse::<u8>()
                .unwrap_or(0);

            if major == 0 && minor == 0 && bug == 0 {
                Err(1)
            } else {
                Ok(Version { minor, major, bug })
            }
        } else {
            Err(0)
        }
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
    pub module: String,
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
    pub module_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub module_map: Vec<ModuleMap>,
    pub debug_headers: Vec<DebugHeader>,
}
