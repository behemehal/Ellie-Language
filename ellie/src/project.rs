use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EllieProject {
    pub name: String,
    pub version: String,
    /// Relative path to the entry .ei file (e.g. "src/main.ei")
    pub entry: String,
    /// Output directory for compiled artifacts (e.g. "output")
    pub output: String,
    /// Bytecode generation: 2 = gen2 (default), 1 = gen1
    pub gen: u8,
    /// Future: library dependencies
    pub libs: Vec<String>,
    /// Paths to compiled native bridge .dll/.so files
    pub bridges: Vec<String>,
}

impl Default for EllieProject {
    fn default() -> Self {
        EllieProject {
            name: String::new(),
            version: "0.1.0".to_string(),
            entry: "src/main.ei".to_string(),
            output: "output".to_string(),
            gen: 2,
            libs: vec![],
            bridges: vec![],
        }
    }
}

pub const PROJECT_FILE: &str = "ellie.json";

/// Load and parse `ellie.json` from the given project root directory.
pub fn load(project_dir: &Path) -> Result<EllieProject, String> {
    let path = project_dir.join(PROJECT_FILE);
    let data = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read {}: {}", path.display(), e))?;
    serde_json::from_str::<EllieProject>(&data)
        .map_err(|e| format!("Invalid {}: {}", PROJECT_FILE, e))
}

/// Write `ellie.json` to the given project root directory.
pub fn save(project_dir: &Path, project: &EllieProject) -> Result<(), String> {
    let path = project_dir.join(PROJECT_FILE);
    let data = serde_json::to_string_pretty(project)
        .map_err(|e| format!("Serialize error: {}", e))?;
    std::fs::write(&path, data)
        .map_err(|e| format!("Cannot write {}: {}", path.display(), e))
}
