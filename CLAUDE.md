# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ellie is a type-safe programming language designed for embedded and sandboxed environments, implemented in Rust (v1.5.20-alpha, codename "Seftali"). The repo contains the full toolchain: compiler, VM, formatter, and CLI binaries.

## Repository Layout

The root `c:\Users\ahmet\Desktop\Ellie-Language` contains:
- **ellie_engine/** — Cargo workspace with all compiler/VM crates
- **elliec/** — Standalone CLI compiler binary
- **ellievm/** — Standalone CLI VM binary
- **elliefmt/** — Standalone CLI formatter binary (incomplete)
- **Ellie-Core-Library/** — Standard library for Ellie programs
- **tools/** — Release scripts, integration test runner, opcode utilities

## Language Reference

Full Ellie syntax is documented in [lang.md](lang.md) at the repository root.

## ByteCode Format Reference

The binary (`.eic`), debug (`.eig`), and assembly listing (`.eia`) output formats are documented in [ByteCodeFormat.md](ByteCodeFormat.md) at the repository root.

## Commands

All Rust commands are run from inside `ellie_engine/` unless otherwise noted.

```powershell
# Build
cargo build --verbose
cargo build --release -p elliec
cargo build --release -p ellievm

# Test
cargo test --verbose
cargo test -p ellie_tokenizer
cargo test -p ellie_parser
cargo test -p ellie_bytecode

# Lint / Format
cargo fmt --all -- --check --verbose   # CI check
cargo fmt --all                         # auto-format
cargo clippy --all

# Benchmarks (criterion)
cargo bench
```

### Running .ei programs

`run.ps1` / `run.sh` live in `tools/` and must be run from that directory. They compile then immediately execute:

```powershell
# From tools/ — compile then run integration test
cargo run --manifest-path=../elliec/Cargo.toml -- compile integration_files/main.ei -s -e -o byteCode -p integration_files/main
cargo run --manifest-path=../ellievm/Cargo.toml -- run integration_files/main -d integration_files/main.eig -a

# General pattern (from tools/)
cargo run --manifest-path=../elliec/Cargo.toml -- compile <file.ei> -s -e -o byteCode -p <output-stem>
cargo run --manifest-path=../ellievm/Cargo.toml -- run <output-stem> -d <output-stem>.eig -a
```

Key elliec flags: `-s` show debug lines, `-e` exclude std library (use when `.ei` imports its own `core.ei`), `-o byteCode` produces `<stem>` + `<stem>.eig`. The `elliec` bytecode assembler is `ellie_bytecode` (gen1); `ellie_bytecode_gen2` is not yet complete.

### Test suite

`tools/tests/` holds categorized `.ei` test programs. Each category has its own subdirectory; all files import `../core.ei`. Run a single test with the helper scripts (from `tools/`):

```powershell
# PowerShell
./run-test.ps1 math/fib
./run-test.ps1 math/factorial
./run-test.ps1 ffi/windows

# bash
./run-test.sh math/fib
```

Categories so far: `math/` (fib, factorial, primes, sum, power), `ffi/` (windows — kernel32 + user32 calls).

Note: `run.ps1` / `run.sh` assume the standard library and use slightly different flags; use the manual commands above for the integration files.

## Compilation Pipeline

Source code flows through four sequential stages:

```
.ei  →  Tokenizer  →  Parser  →  Bytecode Generator  →  VM
```

1. **ellie_tokenizer** — converts raw text into tokens with source positions
2. **ellie_parser** — builds a typed AST; validates the type system
3. **ellie_bytecode / ellie_bytecode_gen2** — lowers AST to bytecode (`.eic`) and debug info (`.eig`)
4. **ellie_vm / ellie_vm_gen2** — executes `.eic` bytecode

**Gen2 components** (`bytecode_gen2`, `vm_gen2`) are the active development targets. Gen2 uses a register machine ISA (A,B,C,X,Y,SP,FP,PC) with fixed-size instructions, a `Vec<RawType>` data stack, shadow call stack for return addresses, and a heap for strings. Gen1 uses stack-based execution; gen2 is a register machine.

### Gen2 compile + run workflow

```powershell
# From tools/ — compile gen2 and run
cargo run --manifest-path=../elliec/Cargo.toml -- compile tests/gen2/hello.ei -e -o byteCode2 -p tests/gen2/hello.eic2
cargo run --manifest-path=../ellievm/Cargo.toml -- run tests/gen2/hello.eic2
```

The ellievm binary auto-detects gen2 by the `.eic2` extension. Gen1 binaries use no extension or `.eic`.

Gen2 assembly listing (for debugging):
```powershell
cargo run --manifest-path=../elliec/Cargo.toml -- compile tests/gen2/hello.ei -e -o byteCodeAsm2 -p tests/gen2/hello.eia2
```

## Cargo Workspace Features

`ellie_engine/Cargo.toml` defines the feature gates that control which stages are compiled:

| Feature | Enables |
|---|---|
| `compiler` | tokenizer + parser + both bytecode generators |
| `vm` | gen1 VM |
| `vm_gen2` | gen2 VM (`ellie_vm_gen2`) |
| `fmt` | formatter |
| `std` | standard library support |
| `renderer_utils` | CLI error rendering |
| `standard_rules` | optional parser rules |
| `alternate_alloc` | mimalloc instead of system allocator |

Default features: `vm`, `compiler`, `fmt`, `std`.

## build.rs

The workspace `build.rs` auto-generates `src/engine_constants.rs` at compile time. It reads every crate's `Cargo.toml`, extracts version strings, and captures git commit/branch/date. Constants are `ELLIE_ENGINE_VERSION`, `ELLIE_TOKENIZER_VERSION`, etc. Do not hand-edit `engine_constants.rs`.

## File Extensions

- `.ei` — Ellie source
- `.eic` — compiled bytecode (binary, input to ellievm)
- `.eig` — debug info (generated alongside `.eic`)

## Debugging & Development Flags

Pass these to `elliec` when debugging compiler behavior:

- `--show-debug-lines` — show parser line annotations
- `--allow-panics` — expose Rust panic stack traces instead of graceful errors
- `--disable-warnings` — suppress syntax warnings
- `--disable-colors` (short: `-n`) — strip ANSI color codes from output (use in debug/CI runs to get clean plain-text errors)
- `viewModule <file.eic>` — inspect a compiled binary module

## Tools Directory

- **run.ps1 / run.sh** — compile `.ei` → `.eic` then immediately run with `ellievm`
- **release.js** — full release build (requires Deno ≥ 1.38.0)
- **reAssembler.js** — regenerates `instruction_table.rs`, `instructions.md`, and `utils.rs` from `instructions.csv`; run this whenever opcodes change
- **tools/integration_test/** — end-to-end Cargo crate that exercises the full compiler+VM pipeline

## CI

GitHub Actions (`.github/workflows/rust.yml`) runs on push/PR to `main`. Working directory is `ellie_engine/`. Steps: build → fmt check. Tests are currently commented out in CI.
