use crate::project::{EllieProject, PROJECT_FILE};
use std::{fs, path::Path};

// ── Platform-specific dylib filename ─────────────────────────────────────────

#[cfg(target_os = "windows")]
fn dylib_filename(name: &str) -> String {
    format!("{}.dll", name.replace('-', "_"))
}

#[cfg(target_os = "macos")]
fn dylib_filename(name: &str) -> String {
    format!("lib{}.dylib", name.replace('-', "_"))
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn dylib_filename(name: &str) -> String {
    format!("lib{}.so", name.replace('-', "_"))
}

// ── Name helpers ──────────────────────────────────────────────────────────────

fn pascal_case(s: &str) -> String {
    s.split('-')
        .flat_map(|seg| seg.split('_'))
        .map(|seg| {
            let mut c = seg.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

// ── File templates ────────────────────────────────────────────────────────────

fn main_ei_template() -> &'static str {
    r#"import "./core.ei";

fn main() {
    println("Hello, World!");
}
"#
}

fn lib_ei_template(name: &str, dylib: &str) -> String {
    let class_name = pascal_case(name);
    format!(
        r#"import "./core.ei";

// {name} — native bridge library.
// init_bridge() loads the compiled native library and returns a NativeBridge handle.

pub class {class_name} {{
    co() {{
        self.lib = init_bridge("{dylib}");
    }}
    pri v lib: NativeBridge;

    // Example: wraps the "add" function exported by the Rust crate.
    pub fn add(a: int, b: int) : int {{
        ret self.lib.call("add", [("int", a), ("int", b)]) as int;
    }}
}}
"#,
        name = name,
        class_name = class_name,
        dylib = dylib,
    )
}

fn build_ei_template(name: &str) -> String {
    format!(
        r#"import "./core.ei";

// build.ei — project build script.
// The ellie toolchain runs build() before compiling your project.

fn build() {{
    if !command_exists("cargo") {{
        panic("cargo is required to build native bridges");
    }}
    v code = run_command("cargo", ["build", "--release", "--manifest-path", "bridges/{name}/Cargo.toml"]);
    if code != 0 {{
        panic("Native bridge build failed");
    }}
    println("Build complete");
}}
"#,
        name = name
    )
}

fn bridge_cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
ellie_native_bridge = {{ path = "../../../EllieNativeBridge", features = ["rust_bridge"] }}
"#,
        name = name
    )
}

fn bridge_lib_rs(name: &str) -> String {
    format!(
        r#"use ellie_native_bridge::rust::{{
    EllieFunction, EllieModule, FunctionAnswer, FunctionCallParameter,
}};

#[no_mangle]
pub extern "Rust" fn add(args: Vec<FunctionCallParameter>) -> FunctionAnswer {{
    if args.len() != 2 {{
        return FunctionAnswer::RuntimeError("Expected 2 arguments".to_string());
    }}
    let left = match args[0].data.clone().into_integer() {{
        Ok(i) => i.as_isize,
        Err(_) => return FunctionAnswer::RuntimeError("Expected integer".to_string()),
    }};
    let right = match args[1].data.clone().into_integer() {{
        Ok(i) => i.as_isize,
        Err(_) => return FunctionAnswer::RuntimeError("Expected integer".to_string()),
    }};
    FunctionAnswer::Ok((left + right).into())
}}

#[no_mangle]
pub extern "Rust" fn load_module() -> EllieModule {{
    EllieModule {{
        name: "{name}",
        version: "0.1.0",
        functions: vec![EllieFunction {{
            name: "add",
            on_call: add,
        }}],
    }}
}}
"#,
        name = name
    )
}

// ── Shared native bridge scaffolding ──────────────────────────────────────────

/// Creates `bridges/<name>/Cargo.toml` and `bridges/<name>/src/lib.rs` inside `dest`.
fn scaffold_native_bridge(name: &str, dest: &Path) -> Result<String, String> {
    let dylib = format!("bridges/{}/target/release/{}", name, dylib_filename(name));
    let bridge_src = dest.join("bridges").join(name).join("src");
    fs::create_dir_all(&bridge_src)
        .map_err(|e| format!("Cannot create bridges/{}/src/: {}", name, e))?;
    fs::write(
        dest.join("bridges").join(name).join("Cargo.toml"),
        bridge_cargo_toml(name),
    )
    .map_err(|e| format!("Cannot write bridges/{}/Cargo.toml: {}", name, e))?;
    fs::write(bridge_src.join("lib.rs"), bridge_lib_rs(name))
        .map_err(|e| format!("Cannot write bridges/{}/src/lib.rs: {}", name, e))?;
    Ok(dylib)
}

// ── ellie new <name> ──────────────────────────────────────────────────────────

pub fn new_project(name: &str, dest: &Path) -> Result<(), String> {
    if dest.exists() {
        return Err(format!("Directory '{}' already exists", dest.display()));
    }

    // src/main.ei
    let src = dest.join("src");
    fs::create_dir_all(&src)
        .map_err(|e| format!("Cannot create src/: {}", e))?;
    fs::write(src.join("main.ei"), main_ei_template())
        .map_err(|e| format!("Cannot write src/main.ei: {}", e))?;

    // bridges/<name>/
    let dylib = scaffold_native_bridge(name, dest)?;

    // build.ei at project root
    fs::write(dest.join("build.ei"), build_ei_template(name))
        .map_err(|e| format!("Cannot write build.ei: {}", e))?;

    // ellie.json
    let project = EllieProject {
        name: name.to_string(),
        bridges: vec![dylib],
        ..EllieProject::default()
    };
    crate::project::save(dest, &project)?;

    // .gitignore
    fs::write(dest.join(".gitignore"), "/output\n/bridges/*/target\n")
        .map_err(|e| format!("Cannot write .gitignore: {}", e))?;

    // output/
    fs::create_dir_all(dest.join("output"))
        .map_err(|e| format!("Cannot create output/: {}", e))?;

    println!("Created project '{}' at {}", name, dest.display());
    println!("  {}  — entry point",   dest.join("src/main.ei").display());
    println!("  {}  — build script",  dest.join("build.ei").display());
    println!("  {}  — Rust bridge",   dest.join("bridges").join(name).display());
    println!("  {}  — project config", dest.join(PROJECT_FILE).display());
    println!();
    println!("Build the native bridge, then run:");
    println!("  cd {}", dest.join("bridges").join(name).display());
    println!("  cargo build --release");
    println!("  cd {}", dest.display());
    println!("  ellie run");

    Ok(())
}

// ── ellie bridge new <name> ───────────────────────────────────────────────────

pub fn new_bridge(name: &str, dest: &Path) -> Result<(), String> {
    if dest.exists() {
        return Err(format!("Directory '{}' already exists", dest.display()));
    }

    // bridges/<name>/ — compute dylib path before writing src/lib.ei
    let dylib = scaffold_native_bridge(name, dest)?;

    // src/lib.ei
    let src = dest.join("src");
    fs::create_dir_all(&src)
        .map_err(|e| format!("Cannot create src/: {}", e))?;
    fs::write(src.join("lib.ei"), lib_ei_template(name, &dylib))
        .map_err(|e| format!("Cannot write src/lib.ei: {}", e))?;

    // build.ei at project root
    fs::write(dest.join("build.ei"), build_ei_template(name))
        .map_err(|e| format!("Cannot write build.ei: {}", e))?;

    // ellie.json
    let project = EllieProject {
        name: name.to_string(),
        entry: "src/lib.ei".to_string(),
        bridges: vec![dylib],
        ..EllieProject::default()
    };
    crate::project::save(dest, &project)?;

    // .gitignore
    fs::write(dest.join(".gitignore"), "/output\n/bridges/*/target\n")
        .map_err(|e| format!("Cannot write .gitignore: {}", e))?;

    // output/
    fs::create_dir_all(dest.join("output"))
        .map_err(|e| format!("Cannot create output/: {}", e))?;

    println!("Created bridge project '{}' at {}", name, dest.display());
    println!("  {}  — Ellie library source", dest.join("src/lib.ei").display());
    println!("  {}  — build script",         dest.join("build.ei").display());
    println!("  {}  — Rust bridge crate",    dest.join("bridges").join(name).display());
    println!("  {}  — project config",       dest.join(PROJECT_FILE).display());
    println!();
    println!("Build the native bridge first:");
    println!("  cd {}", dest.join("bridges").join(name).display());
    println!("  cargo build --release");
    println!();
    println!("Then run the Ellie project:");
    println!("  cd {}", dest.display());
    println!("  ellie run");

    Ok(())
}
