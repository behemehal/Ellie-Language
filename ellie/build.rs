// Builds and ships ellie_core_native alongside the ellie binary on release.
//
// Debug builds skip this — the runtime falls back to inline L1 implementations
// in build_core_module() when no core dylib is present. Set
// ELLIE_BUILD_CORE=1 to force a build in any profile, ELLIE_SKIP_CORE_BUILD=1
// to skip it even on release.

use std::path::Path;
use std::process::Command;

fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_default();
    let force = std::env::var_os("ELLIE_BUILD_CORE").is_some();
    let skip = std::env::var_os("ELLIE_SKIP_CORE_BUILD").is_some();

    println!("cargo:rerun-if-env-changed=ELLIE_BUILD_CORE");
    println!("cargo:rerun-if-env-changed=ELLIE_SKIP_CORE_BUILD");

    if skip || (profile != "release" && !force) {
        return;
    }

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let project_root = Path::new(&manifest_dir)
        .parent()
        .expect("ellie crate has no parent dir");
    let core_native_dir = project_root.join("Ellie-Core-Library").join("native");

    if !core_native_dir.exists() {
        println!(
            "cargo:warning=ellie_core_native not found at {} — skipping core dylib build",
            core_native_dir.display()
        );
        return;
    }

    println!(
        "cargo:rerun-if-changed={}",
        core_native_dir.join("Cargo.toml").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        core_native_dir.join("src").join("lib.rs").display()
    );

    let target = std::env::var("TARGET").unwrap_or_default();

    let mut cmd = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
    cmd.arg("build").arg("--release");
    if !target.is_empty() {
        cmd.arg("--target").arg(&target);
    }
    cmd.current_dir(&core_native_dir);

    let status = cmd
        .status()
        .expect("failed to invoke cargo for ellie_core_native");
    if !status.success() {
        panic!(
            "ellie_core_native build failed with status {:?}; \
             set ELLIE_SKIP_CORE_BUILD=1 to bypass",
            status
        );
    }

    let core_release_dir = if target.is_empty() {
        core_native_dir.join("target").join("release")
    } else {
        core_native_dir.join("target").join(&target).join("release")
    };

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let artifact_name = match target_os.as_str() {
        "windows" => "ellie_core_native.dll",
        "macos" => "libellie_core_native.dylib",
        _ => "libellie_core_native.so",
    };

    let src = core_release_dir.join(artifact_name);
    if !src.exists() {
        panic!(
            "expected ellie_core_native artifact not found at {}",
            src.display()
        );
    }

    // OUT_DIR shape: <target>/<maybe_triple>/<profile>/build/ellie-<hash>/out
    // Walk up four ancestors to land at <target>/<maybe_triple>/<profile>/,
    // the same directory where the ellie binary will be written.
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let ellie_release_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .expect("could not derive ellie release dir from OUT_DIR");

    let dst = ellie_release_dir.join(artifact_name);
    std::fs::copy(&src, &dst).unwrap_or_else(|e| {
        panic!(
            "failed to copy {} -> {}: {}",
            src.display(),
            dst.display(),
            e
        )
    });

    println!(
        "cargo:warning=ellie: shipped core native dylib at {}",
        dst.display()
    );

    // ── Ship the runtime stdlib bundle ───────────────────────────────────────
    // The compiler resolves `import "std"` / `import "std/<file>"` to this dir.
    // Source is `Ellie-Core-Library/runtime/` — a curated bundle that the
    // gen2 parser handles end-to-end. The full `Ellie-Core-Library/lib/` is
    // the long-term goal; once gen2 supports its full syntax we point at it
    // instead.

    let stdlib_src = project_root.join("Ellie-Core-Library").join("runtime");
    if !stdlib_src.is_dir() {
        println!(
            "cargo:warning=ellie: stdlib runtime bundle not found at {} — skipping",
            stdlib_src.display()
        );
        return;
    }

    let stdlib_dst = ellie_release_dir.join("std");
    let _ = std::fs::remove_dir_all(&stdlib_dst);
    std::fs::create_dir_all(&stdlib_dst).unwrap_or_else(|e| {
        panic!("failed to create {}: {}", stdlib_dst.display(), e)
    });

    let mut copied = 0usize;
    for entry in std::fs::read_dir(&stdlib_src)
        .unwrap_or_else(|e| panic!("read_dir {}: {}", stdlib_src.display(), e))
    {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("ei") {
            continue;
        }
        let file_name = match path.file_name() {
            Some(n) => n,
            None => continue,
        };
        let dst_path = stdlib_dst.join(file_name);
        std::fs::copy(&path, &dst_path).unwrap_or_else(|e| {
            panic!("failed to copy {} -> {}: {}", path.display(), dst_path.display(), e)
        });
        println!("cargo:rerun-if-changed={}", path.display());
        copied += 1;
    }

    println!(
        "cargo:warning=ellie: shipped {} stdlib .ei file(s) at {}",
        copied,
        stdlib_dst.display()
    );
}
