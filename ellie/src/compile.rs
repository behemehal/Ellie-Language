use ellie_engine::{
    compiler::parse_pages,
    ellie_bytecode_gen2::assembler::{Assembler, PlatformAttributes},
    ellie_core::{
        defs::{PlatformArchitecture, Version},
        module_path::parse_module_import,
    },
    ellie_renderer_utils::utils::{print_errors, print_warnings, read_file, CliColor, ColorDisplay},
    ellie_tokenizer::tokenizer::{ImportType, ResolvedImport},
    tokenizer,
    utils::{CompilerSettings, MainProgram, ProgramRepository},
};
use path_absolutize::Absolutize;
use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

/// Locate the bundled stdlib directory shipped next to the running ellie binary.
/// Looks for `std/` adjacent to the executable; returns None if missing.
fn bundled_stdlib_dir() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    let candidate = dir.join("std");
    if candidate.is_dir() {
        Some(candidate)
    } else {
        None
    }
}

/// Read a resolved file into a ResolvedImport. Returns a not-found result on read error.
fn resolved_from_path(path: &Path) -> ResolvedImport {
    use ellie_engine::ellie_renderer_utils::utils::read_file;
    match read_file(path.to_string_lossy().to_string()) {
        Ok(data) => {
            let mut h = DefaultHasher::new();
            data.hash(&mut h);
            ResolvedImport {
                found: true,
                matched: ImportType::Code(data),
                hash: (h.finish() as u32) as usize,
                path: path.to_string_lossy().into_owned(),
                ..Default::default()
            }
        }
        Err(_) => ResolvedImport {
            found: false,
            resolve_error: format!("Cannot read file: {}", path.display()),
            ..Default::default()
        },
    }
}

/// Compile a gen2 `.ei` file.
/// Returns paths to the written `.eic2` and `.eig` files on success.
pub fn compile_gen2(
    target_path: &Path,
    output_dir: &Path,
    project_name: &str,
    exclude_std: bool,
) -> Result<(PathBuf, PathBuf), ()> {
    let cli_color = CliColor;

    let file_name = target_path
        .file_name().unwrap().to_str().unwrap().to_string();

    let stem = target_path
        .file_stem().unwrap().to_str().unwrap().to_string();

    let out_eic2 = output_dir.join(format!("{}.eic2", stem));
    let out_eig  = output_dir.join(format!("{}.eig",  stem));

    // ── Repository impl ───────────────────────────────────────────────────
    struct Repo {
        target_path: String,
        project_name: String,
        main_hash: usize,
    }

    impl ProgramRepository for Repo {
        fn read_main(&mut self) -> MainProgram {
            match read_file(self.target_path.clone()) {
                Ok(content) => {
                    let mut h = DefaultHasher::new();
                    content.hash(&mut h);
                    let hash = (h.finish() as u32) as usize;
                    self.main_hash = hash;
                    MainProgram {
                        file_content: content,
                        file_name: Path::new(&self.target_path)
                            .file_name().unwrap().to_str().unwrap().to_string(),
                        file_hash: hash,
                        start_directory: format!("<ellie_module_{}>", self.project_name),
                    }
                }
                Err(e) => {
                    eprintln!("Cannot read '{}': {}", self.target_path, e);
                    std::process::exit(1);
                }
            }
        }

        fn read_module(
            &mut self,
            link_module: bool,
            current_path: String,
            requested_path: String,
        ) -> ResolvedImport {
            if link_module {
                return ResolvedImport {
                    found: false,
                    resolve_error: "Module linking not supported".to_string(),
                    ..Default::default()
                };
            }

            let stdlib_dir = bundled_stdlib_dir();

            // ── Stdlib root imports: `import "std"` or `import "std/<file>"` ───
            if let Some(ref stdlib) = stdlib_dir {
                let stdlib_rel: Option<String> = if requested_path == "std" {
                    Some("lib.ei".to_string())
                } else if let Some(rest) = requested_path.strip_prefix("std/") {
                    Some(rest.to_string())
                } else {
                    None
                };

                if let Some(rel) = stdlib_rel {
                    let full = stdlib.join(&rel);
                    if full.exists() {
                        return resolved_from_path(&full);
                    } else {
                        return ResolvedImport {
                            found: false,
                            resolve_error: format!(
                                "stdlib file not found: {}",
                                full.display()
                            ),
                            ..Default::default()
                        };
                    }
                }
            }

            // ── Relative import inside the stdlib ──────────────────────────────
            // When a stdlib file imports another stdlib file with `./X.ei`,
            // current_path is the absolute filesystem path of the importer
            // (set by `resolved_from_path`). Resolve relative to its parent.
            if let Some(ref stdlib) = stdlib_dir {
                let cp = Path::new(&current_path);
                if cp.starts_with(stdlib) {
                    let parent = cp.parent().unwrap_or(stdlib);
                    let joined = parent.join(&requested_path);
                    let resolved = joined
                        .absolutize()
                        .map(|p| p.into_owned())
                        .unwrap_or(joined);
                    if resolved.exists() {
                        return resolved_from_path(&resolved);
                    } else {
                        return ResolvedImport {
                            found: false,
                            resolve_error: format!(
                                "stdlib relative import not found: {}",
                                resolved.display()
                            ),
                            ..Default::default()
                        };
                    }
                }
            }

            // ── Project-relative resolution (existing behavior) ────────────────
            let starter = format!("<ellie_module_{}>", self.project_name);
            match parse_module_import(&current_path, &requested_path) {
                Ok(path) => {
                    let real = path.replace(
                        &starter,
                        Path::new(&self.target_path)
                            .absolutize().unwrap()
                            .parent().unwrap()
                            .to_str().unwrap(),
                    );
                    if Path::new(&real).exists() {
                        match read_file(real) {
                            Ok(data) => {
                                let mut h = DefaultHasher::new();
                                data.hash(&mut h);
                                ResolvedImport {
                                    found: true,
                                    matched: ImportType::Code(data),
                                    hash: (h.finish() as u32) as usize,
                                    path,
                                    ..Default::default()
                                }
                            }
                            Err(_) => ResolvedImport {
                                found: false,
                                resolve_error: "Cannot read file".to_string(),
                                ..Default::default()
                            },
                        }
                    } else {
                        ResolvedImport {
                            found: false,
                            resolve_error: format!("Path not found: {}", real),
                            ..Default::default()
                        }
                    }
                }
                Err(_) => ResolvedImport {
                    found: false,
                    resolve_error: "Import resolution failed".to_string(),
                    ..Default::default()
                },
            }
        }
    }

    let mut repo = Repo {
        target_path: target_path.to_str().unwrap().to_string(),
        project_name: project_name.to_string(),
        main_hash: 0,
    };

    // ── Tokenize ──────────────────────────────────────────────────────────
    let tokenized = match tokenizer::tokenize_file(&mut repo) {
        Ok(t) => t,
        Err(errors) => {
            let out = print_errors(
                &errors,
                |path| read_file(path).unwrap_or_default(),
                false,
                |path| read_file(path).unwrap_or_default(),
                cli_color,
            );
            eprint!("{}", out);
            return Err(());
        }
    };

    // ── Parse ─────────────────────────────────────────────────────────────
    let version = Version::build_from_string(
        &ellie_engine::engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
    );
    let compiler_settings = CompilerSettings {
        name: project_name.to_string(),
        file_name: file_name.clone(),
        is_lib: false,
        description: String::new(),
        experimental_features: false,
        version,
        byte_code_architecture: PlatformArchitecture::B64,
    };

    let compile_output = match parse_pages(repo.main_hash, vec![], tokenized, compiler_settings) {
        Ok(out) => out,
        Err(errors) => {
            let out = print_errors(
                &errors,
                |path| read_file(path).unwrap_or_default(),
                false,
                |path| read_file(path).unwrap_or_default(),
                cli_color,
            );
            eprint!("{}", out);
            return Err(());
        }
    };

    if !compile_output.warnings.is_empty() {
        let out = print_warnings(
            &compile_output.warnings,
            |path| read_file(path).unwrap_or_default(),
            |path| read_file(path).unwrap_or_default(),
            cli_color,
        );
        eprint!("{}", out);
    }

    // ── Assemble gen2 ─────────────────────────────────────────────────────
    let mut assembler = Assembler::new(
        compile_output.module,
        PlatformAttributes {
            architecture: PlatformArchitecture::B64,
            memory_size: 0x100000,
        },
    );
    let result = assembler.assemble(vec![]);

    // ── Write outputs ─────────────────────────────────────────────────────
    fs::create_dir_all(output_dir).map_err(|e| eprintln!("Cannot create output dir: {}", e))?;

    let bytecode = result.render_binary_to_vector();
    fs::write(&out_eic2, &bytecode)
        .map_err(|e| eprintln!("Cannot write {}: {}", out_eic2.display(), e))?;

    // Debug info: render_binary writes both bytecode and debug side-by-side;
    // we only want the debug portion. Use a separate debug writer.
    let mut eic2_buf: Vec<u8> = Vec::new();
    let mut eig_buf: Vec<u8> = Vec::new();
    result.render_binary(&mut eic2_buf, &mut eig_buf);
    fs::write(&out_eig, &eig_buf)
        .map_err(|e| eprintln!("Cannot write {}: {}", out_eig.display(), e))?;

    println!(
        "{}[!]{}: Compiled  {}",
        cli_color.color(ellie_engine::ellie_renderer_utils::utils::Colors::Green),
        cli_color.color(ellie_engine::ellie_renderer_utils::utils::Colors::Reset),
        out_eic2.display()
    );

    Ok((out_eic2, out_eig))
}
