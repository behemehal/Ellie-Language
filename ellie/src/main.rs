mod compile;
mod project;
mod run_gen2;
mod scaffold;

use ellie_engine::ellie_renderer_utils::options;
use std::{
    path::{Path, PathBuf},
    process,
};

fn version() {
    println!(
        "ellie {} — Ellie language toolchain",
        env!("CARGO_PKG_VERSION")
    );
}

fn compile_and_run(entry: &Path, output_dir: &Path, project_name: &str, bridges: &[PathBuf]) {
    if !entry.exists() {
        eprintln!("Error: entry file '{}' not found", entry.display());
        process::exit(1);
    }
    match compile::compile_gen2(entry, output_dir, project_name, true) {
        Ok((eic2, _eig)) => {
            let program = load_program(&eic2);
            run_gen2::run(program, bridges);
        }
        Err(()) => process::exit(1),
    }
}

fn compile_only(entry: &Path, output_dir: &Path, project_name: &str) {
    if !entry.exists() {
        eprintln!("Error: entry file '{}' not found", entry.display());
        process::exit(1);
    }
    if let Err(()) = compile::compile_gen2(entry, output_dir, project_name, true) {
        process::exit(1);
    }
}

fn load_program(eic2: &Path) -> ellie_engine::ellie_vm_gen2::program::Program {
    let bytes = std::fs::read(eic2).unwrap_or_else(|e| {
        eprintln!("Cannot read '{}': {}", eic2.display(), e);
        process::exit(1);
    });
    ellie_engine::ellie_vm_gen2::program::Program::load_from_bytes(&bytes).unwrap_or_else(|e| {
        eprintln!("Cannot parse program '{}': {}", eic2.display(), e);
        process::exit(1);
    })
}

fn project_name_from_path(p: &Path) -> String {
    p.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("program")
        .to_string()
}

fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn main() {
    let app = options::generate_ellie_options();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("version", _)) => version(),

        Some(("new", m)) => {
            let name = m.value_of("name").unwrap();
            let dest = match m.value_of("path") {
                Some(p) => PathBuf::from(p).join(name),
                None => PathBuf::from(name),
            };
            if let Err(e) = scaffold::new_project(name, &dest) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }

        Some(("bridge", m)) => match m.subcommand() {
            Some(("new", m)) => {
                let name = m.value_of("name").unwrap();
                let base = m.value_of("path").map(PathBuf::from).unwrap_or_else(cwd);
                let dest = base.join(name);
                if let Err(e) = scaffold::new_bridge(name, &dest) {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
            _ => unreachable!("clap should ensure we don't get here"),
        },

        Some(("run", m)) => match m.value_of("file") {
            Some(f) => {
                let f = PathBuf::from(f);
                let entry = f.canonicalize().unwrap_or(f.clone());
                let output_dir = entry.parent().unwrap_or(Path::new(".")).join("output");
                let name = project_name_from_path(&f);
                compile_and_run(&entry, &output_dir, &name, &[]);
            }
            None => {
                let root = m.value_of("path").map(PathBuf::from).unwrap_or_else(cwd);
                let proj = project::load(&root).unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    eprintln!(
                        "Tip: run `ellie new <name>` to create a project, or `ellie run <file.ei>` to run a script."
                    );
                    process::exit(1);
                });
                let entry = root.join(&proj.entry);
                let output_dir = root.join(&proj.output);
                let bridges: Vec<PathBuf> = proj.bridges.iter().map(|b| root.join(b)).collect();
                compile_and_run(&entry, &output_dir, &proj.name, &bridges);
            }
        },

        Some(("build", m)) => match m.value_of("file") {
            Some(f) => {
                let f = PathBuf::from(f);
                let entry = f.canonicalize().unwrap_or(f.clone());
                let output_dir = entry.parent().unwrap_or(Path::new(".")).join("output");
                let name = project_name_from_path(&f);
                compile_only(&entry, &output_dir, &name);
            }
            None => {
                let root = m.value_of("path").map(PathBuf::from).unwrap_or_else(cwd);
                let proj = project::load(&root).unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                });
                let entry = root.join(&proj.entry);
                let output_dir = root.join(&proj.output);
                compile_only(&entry, &output_dir, &proj.name);
            }
        },

        _ => unreachable!("clap should ensure we don't get here"),
    }
}
