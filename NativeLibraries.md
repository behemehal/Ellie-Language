# Native Libraries

Design and reference for how native code interfaces with the Ellie VM —
covers the bridge ABI, the std-library native surface (networking, processes,
filesystem, etc.), raw FFI, and the tooling Ellie ships for authors of C/Rust
native libraries.

Status: **draft / design**. Some of what's described here is implemented in
`EllieNativeBridge/` and `Ellie-Core-Library/native/`; the rest is the target
shape. See [Roadmap](#11-roadmap) for what exists vs. what's planned.

> **Design philosophy**
> Rust gives us strictness — explicit manifests, typed ABI, target triples,
> capability declarations. Node gives us ergonomics — `import "tcp"` and it
> just works, one build command, high-level APIs over syscalls. We want both.

---

## Table of contents

1. [Layers](#1-layers)
2. [Primary ABI: Rust](#2-primary-abi-rust)
3. [Secondary ABI: C](#3-secondary-abi-c)
4. [Module discovery and loading](#4-module-discovery-and-loading)
5. [Manifest: `lia.yaml`](#5-manifest-liayaml)
6. [Standard library native surface](#6-standard-library-native-surface)
7. [Raw FFI vs. native modules](#7-raw-ffi-vs-native-modules)
8. [Authoring tools (`ellie native ...`)](#8-authoring-tools-ellie-native-)
9. [Memory and ownership rules](#9-memory-and-ownership-rules)
10. [Security and capabilities](#10-security-and-capabilities)
11. [Roadmap](#11-roadmap)

---

## 1. Layers

Native code reaches the VM through four distinct layers. Keeping them
distinct is important — each has different stability, security, and
performance trade-offs.

| Layer | What it is | Stability | Authored in | Loaded how |
|---|---|---|---|---|
| **L1 — Inline VM fallbacks** | Functions implemented inside the `ellie` binary itself, used only when no loaded bridge claims the name. Identical to L2 from the program's point of view; exist so the VM still works if the core native cdylib is missing. | Tied to `ellie` binary version | Rust, inside `ellie` | Compiled in |
| **L2 — Core native module** | `Ellie-Core-Library/native/` — a `cdylib` that provides std-library implementations. Same ABI as L3, just bundled with the language. | Tied to engine version | Rust (uses bridge) | Auto-loaded next to the `ellie` binary at startup |
| **L3 — Third-party native modules** | User-authored `cdylib`s using the canonical bridge ABI. Networking sockets, database drivers, sandbox-specific glue, etc. | Versioned ABI | Rust or C | Loaded by manifest or `import`; bridge paths passed to the runner |
| **L4 — Raw FFI** | `load_library_handle()` + `call_library_function()` — call arbitrary C ABI libraries (kernel32, libsqlite3, etc.) by name. | No safety net | Any C-ABI lib | Loaded ad-hoc by program |

**Rules of thumb:**

- Use **L1** only when something needs VM internals (heap layout, scheduler)
  or as a safety net so basic programs still run when the core cdylib hasn't
  been built yet. New std-library functions should land in L2, not L1.
- Use **L2** for anything that ships with the language (`http`, `fs`, `net`).
- Use **L3** when an Ellie program needs a typed, Ellie-friendly module that
  wraps native code (a postgres driver, an image codec, an ML runtime).
- Use **L4** as the escape hatch — when you need to call into a library
  whose authors have never heard of Ellie.

---

## 2. Primary ABI: Rust

The primary bridge ABI is **Rust-native**. Native libs export
`extern "Rust" fn load_module() -> EllieModule` where `EllieModule` is the
plain Rust struct from `ellie_native_bridge::rust`. Args and return values
are rich Rust types (`String`, `Vec<EllieData>`, `enum EllieData`). No C
representation in the middle.

This is what [load_bridge()](ellie/src/run_gen2.rs#L18) already does today.
The doc cements that as the canonical path.

### 2.1 Why Rust-native is the right primary

- **Zero conversion overhead for Rust libs.** Engine is Rust, core lib is
  Rust, ~all third-party libs will be Rust. Going through a `#[repr(C)]`
  intermediate just to come back to `String`/`Vec<>` on the other side is
  pure cost.
- **The ABI stability problem doesn't apply.** `extern "Rust"` is "unstable"
  only across rustc versions for *binary-distributed* libraries. The core
  lib ships **alongside** the `ellie` binary, built together. Third-party
  Rust libs distribute as **source** (Cargo-style) and get compiled with the
  host's rustc when installed. Mismatch impossible by construction.
- **Rich types reduce author boilerplate.** `String` instead of "pass me a
  `const char*` + `size_t`, here's the vtable callback to copy it." Authors
  spend time on the function logic, not the bridge plumbing.

### 2.2 ABI shape

```rust
pub struct EllieInteger {
    pub as_isize: isize,
    pub as_usize: usize,
}

pub enum EllieData {
    Integer(EllieInteger),
    Float(f32),
    Double(f64),
    Byte(u8),
    Bool(bool),
    String(String),
    Char(char),
    Void,
    Null,
    Array(Vec<EllieData>),
    Class(Vec<EllieData>),
    // additions for v1.0:
    Bytes(Vec<u8>),                                // raw byte buffer
    Handle(EllieHandle),                           // see §9 — native-owned resource
}

pub struct FunctionCallParameter {
    pub data: EllieData,
    pub memory_location: usize,
}

pub enum FunctionAnswer {
    Ok(EllieData),
    RuntimeError(String),
}

pub type FunctionCallback =
    extern "Rust" fn(arguments: Vec<FunctionCallParameter>) -> FunctionAnswer;

pub struct EllieFunction {
    pub name: &'static str,
    pub on_call: FunctionCallback,
}

pub struct EllieModule {
    pub abi_version: u32,                          // == ELLIE_RUST_ABI_VERSION
    pub name: &'static str,
    pub version: &'static str,
    pub functions: Vec<EllieFunction>,
    pub capabilities: Vec<&'static str>,           // see §10
}
```

Diff vs. what's in [EllieNativeBridge/src/rust.rs](EllieNativeBridge/src/rust.rs)
today, all additive:

- `abi_version: u32` at the top of `EllieModule`.
- `EllieData::Bytes(Vec<u8>)` (the current shape has no raw byte buffer —
  for fs/net we need one without paying the per-element enum tag of
  `Array(Vec<EllieData>)`).
- `EllieData::Handle(EllieHandle)` for §9.
- `EllieModule::capabilities` for §10.

Existing libs (the core lib) port over by setting `abi_version` and leaving
everything else alone.

### 2.3 ABI version policy

- `ELLIE_RUST_ABI_VERSION: u32` is the current canonical value. v1.0 is `1`.
- `load_bridge` checks it at load time. Mismatch → clean error, module
  rejected. (Today the field doesn't exist and there's no check.)
- Adding new `EllieData` variants is a breaking change because exhaustive
  matches in older bridges would panic. Bumps the version.
- Adding new fields to `EllieModule` at the end of the struct is a breaking
  change at the `extern "Rust"` level. Bumps the version.
- The `ellie` binary supports the current ABI version. Multi-version
  support is out of scope for v1 — we re-ship native libs together with the
  binary as a unit.

### 2.4 Conventions for authors

- `#[no_mangle] pub extern "Rust" fn load_module() -> EllieModule` is the
  required entry point.
- Library is `cdylib`. (Rust would otherwise demangle and resolve through
  the rlib metadata, which we can't `dlopen`.)
- Each native fn: `#[no_mangle] pub extern "Rust" fn name(args: Vec<FunctionCallParameter>) -> FunctionAnswer`.
- Errors → `FunctionAnswer::RuntimeError("message")`. The VM surfaces this
  as a catchable Ellie error.

A `#[ellie::function]` proc-macro can later be added to elide the
`Vec<FunctionCallParameter>` unpacking, but it's syntactic sugar over this
ABI, not a separate ABI.

---

## 3. Secondary ABI: C

The C ABI exists for native libs that aren't Rust — bindings to existing
C/C++ codebases, libraries written in Zig, anything with a stable C FFI.
**It is a peer ABI, not a substrate** — Rust libs do not go through it.

A native lib declares its ABI choice by which entry point it exports:

| Symbol exported | Loader treats it as |
|---|---|
| `load_module` (`extern "Rust"`) | Rust ABI (§2) |
| `ellie_load_module` (`extern "C"`) | C ABI (this section) |

`load_bridge` tries the Rust symbol first, falls back to the C symbol. A
lib exporting both is undefined behavior — pick one.

### 3.1 Header (`ellie_bridge.h`)

```c
#ifndef ELLIE_BRIDGE_H
#define ELLIE_BRIDGE_H

#include <stdint.h>
#include <stddef.h>

#define ELLIE_C_ABI_VERSION 1

typedef struct EllieCtx EllieCtx;          // opaque, points to VM thread state

typedef enum {
    ELLIE_TY_VOID    = 0,
    ELLIE_TY_NULL    = 1,
    ELLIE_TY_BOOL    = 2,
    ELLIE_TY_BYTE    = 3,
    ELLIE_TY_INT     = 4,
    ELLIE_TY_FLOAT   = 5,
    ELLIE_TY_DOUBLE  = 6,
    ELLIE_TY_CHAR    = 7,
    ELLIE_TY_STRING  = 8,   // heap ref
    ELLIE_TY_ARRAY   = 9,   // heap ref
    ELLIE_TY_BYTES   = 10,  // heap ref to byte buffer
    ELLIE_TY_CLASS   = 11,  // heap ref
    ELLIE_TY_HANDLE  = 12,  // native-owned, see §9
} EllieTypeId;

typedef struct {
    EllieTypeId type_id;
    uint64_t    payload;   // scalars inline, heap types carry an id
} EllieValue;

typedef struct {
    EllieValue value;
    const char* error;     // NULL = success
} EllieAnswer;

typedef EllieAnswer (*EllieFn)(
    EllieCtx*, const EllieValue* args, size_t arg_count
);

typedef struct { const char* name; EllieFn fn; } EllieFunction;

typedef struct {
    uint32_t            abi_version;       // == ELLIE_C_ABI_VERSION
    const char*         name;
    const char*         version;
    const EllieFunction* functions;
    size_t              function_count;
    const char* const*  capabilities;
    size_t              capability_count;
} EllieModule;

const EllieModule* ellie_load_module(EllieCtx* ctx);

// VM-provided vtable — native code goes through this for heap access.
typedef struct {
    const char* (*string_get)(EllieCtx*, uint64_t, size_t* out_len);
    uint64_t    (*string_new)(EllieCtx*, const char*, size_t);

    size_t      (*array_len)(EllieCtx*, uint64_t);
    EllieValue  (*array_get)(EllieCtx*, uint64_t, size_t);
    uint64_t    (*array_new)(EllieCtx*, const EllieValue*, size_t);

    const uint8_t* (*bytes_get)(EllieCtx*, uint64_t, size_t* out_len);
    uint64_t       (*bytes_new)(EllieCtx*, const uint8_t*, size_t);

    EllieValue  (*field_get)(EllieCtx*, uint64_t, size_t);
    void        (*field_set)(EllieCtx*, uint64_t, size_t, EllieValue);

    uint64_t    (*handle_new)(EllieCtx*, void*, void (*drop)(void*));
    void*       (*handle_get)(EllieCtx*, uint64_t);
} EllieVTable;

const EllieVTable* ellie_vtable(EllieCtx*);

#endif
```

### 3.2 Why a vtable on the C side

C code can't safely touch Rust's heap. The vtable is the bridge: every read
or write of a string/array/class goes through a callback the VM provides.
That costs an extra hop per non-scalar access — the price of crossing the
language boundary. Rust libs avoid it entirely by using §2.

### 3.3 Versioning

Independent from the Rust ABI version. `ELLIE_C_ABI_VERSION` starts at `1`.
Same break-on-mismatch policy as §2.3.

---

## 4. Module discovery and loading

### 4.1 Who does the loading

The loader lives in the `ellie` all-in-one binary
([ellie/src/run_gen2.rs](ellie/src/run_gen2.rs)). `ellievm` and `elliec` are
narrower-purpose binaries; in v1.0 they either link the same loader or stay
limited to L1 fallbacks. New std-library work targets the `ellie` binary.

The shape is already in place today — `load_bridge()` `dlopen`s the cdylib,
calls `load_module()`, walks `EllieModule.functions`, and binds each one to
a `Gen2Module` entry. What needs to be added on top of it is:

- the canonical C ABI (§2) instead of the current `extern "Rust"` shape,
- ABI version checking,
- manifest-driven module resolution (§4.3),
- capability enforcement (§10).

### 4.2 Where libraries live

```
~/.ellie/
├── cache/
│   └── <target-triple>/             # x86_64-pc-windows-msvc, aarch64-apple-darwin, ...
│       ├── ellie_core@0.3.0/
│       │   └── libellie_core_native.dylib
│       └── tcp_socket@1.2.0/
│           └── libtcp_socket.dylib
└── registries/
    └── default.json                 # registry index (later)
```

Project-local override: `./.ellie/native/<triple>/<lib>@<ver>/`.

For the core library specifically, the v1.0 path is simpler: `ellie` looks
for `ellie_core_native.{dll|dylib|so}` next to its own binary, the way
`core_native_path()` does today.

### 4.3 Load sequence at program start

1. The compiled program already carries a `native_traces` table — a list of
   `(function_name, function_hash)` entries the bytecode references.
   **This is the source of truth for "what natives this program needs."**
2. `ellie` resolves its host target triple.
3. **Core lib**: locate `ellie_core_native` next to the binary (v1.0) or in
   `~/.ellie/cache/<triple>/ellie_core@<engine-version>/` (v1.4+), `dlopen`,
   try `load_module` (Rust ABI, §2) first then `ellie_load_module` (C ABI,
   §3), check `abi_version`.
4. **User-declared bridges**: from `lia.yaml` dependencies (§4.4) plus any
   `--bridge <path>` args on the CLI, repeat the same load.
5. **Bind**: for each loaded bridge, walk its `EllieFunction[]`. For every
   entry whose `name` appears in `native_traces`, register the function
   under that trace's `function_hash` in the `ModuleManager`. **Unmatched
   bridge functions are ignored** (the lib may export more than the program
   uses); **unmatched native traces fall through to L1**, then become a
   runtime error on first call if L1 has no fallback either.
6. **Conflict policy**: first bridge to claim a name wins; subsequent bridges
   that export the same name log a warning and are skipped for that entry.
   Core lib loads first by convention.

### 4.4 User libraries

In a program's `lia.yaml`:

```yaml
dependencies:
  http: { version: "0.1", source: registry }
  pg:   { version: "2.4", source: git, url: "https://github.com/foo/ellie-pg" }
```

`import "http"` in `.ei` triggers the resolver: it finds the dependency,
ensures the native artifact for the current triple is present in cache
(build it from source on miss), and adds it to the bridge list `ellie` loads
at startup, alongside core. From there the same name → trace binding (§4.3)
takes over.

Until the resolver lands (v1.4), the explicit `--bridge <path>` CLI arg is
the user-facing way to load an L3 module — already plumbed through
`run(program, bridge_paths)`.

### 4.5 Symbol naming

A module declares functions by name (`"add"`). The compiler emits the
matching `@native` declarations into `native_traces` (function name +
collision-resistant hash) as part of the gen2 bytecode. The bridge ABI uses
the plain name; the hash is the VM's internal handle. No symbol mangling.

---

## 5. Manifest: `lia.yaml`

The current manifest is minimal:

```yaml
name: ellieCore
version: 0.3.0
description: Ellie-Core contains core types for Ellie
author: Behemehal
license: GPL-2.0
dependencies:
```

For native libraries, we extend it. **Backwards compatible** — existing pure-Ellie
libraries don't need any new fields.

```yaml
name: tcp_socket
version: 1.2.0
description: TCP sockets for Ellie
author: Someone
license: MIT

# === optional native section ===
native:
  language: rust              # rust | c | cpp | zig
  entry: src/lib.rs           # or CMakeLists.txt / build.zig
  abi_version: 1
  capabilities:               # what the VM is allowed to grant this module
    - net
    - fs.read

  # Targets this lib is published for. Builder will warn if asked for a triple not listed.
  targets:
    - x86_64-pc-windows-msvc
    - x86_64-unknown-linux-gnu
    - aarch64-apple-darwin

  # Optional: extra cargo features / cmake defines per target
  build:
    rust:
      features: [tls]
    c:
      cflags: ["-O2"]

# === ellie-side stubs ===
exports:
  - src/lib.ei                # the .ei file with @native fn declarations
```

The build tool (`ellie native build`) reads this and produces the native
artifact + a metadata file the resolver uses at install time.

---

## 6. Standard library native surface

This is the API the **core library** exposes through L2. It's not a Rust API,
it's the Ellie-facing API — the `.ei` declarations users actually call.
Implementations live in `Ellie-Core-Library/native/`.

> **Concurrency model**: synchronous, OS-thread-based. Anything that would
> block (network read, child process wait) blocks the calling Ellie thread.
> Use `thread.spawn` + `channel` for parallelism. (See §6.6.)

### 6.1 `platform` — already partly exists

Existing: [platform.ei](Ellie-Core-Library/lib/platform.ei). Keep as-is.

- `get_arch() : string`
- `get_os() : string`
- `get_eol() : string`
- `print(s)`, `println(s)`, `panic(s)`
- `timestamp() : int`
- `sleep_ms(ms: int)`

### 6.2 `fs` — filesystem

New. Modeled after Rust's `std::fs` with Node-style convenience methods on top.

```ei
// One-shot helpers (Node-style)
pub fn fs_read_file(path: string) : string;
pub fn fs_read_bytes(path: string) : [byte];
pub fn fs_write_file(path: string, data: string);
pub fn fs_write_bytes(path: string, data: [byte]);
pub fn fs_append(path: string, data: string);
pub fn fs_exists(path: string) : bool;
pub fn fs_remove(path: string);
pub fn fs_rename(from: string, to: string);
pub fn fs_create_dir(path: string);
pub fn fs_create_dir_all(path: string);
pub fn fs_read_dir(path: string) : [string];

// Streaming (Rust-style) — returns an opaque handle
pub class File {
    co(path: string, mode: string);          // "r" "w" "rw" "a"
    pub fn read(n: int) : [byte];
    pub fn read_to_string() : string;
    pub fn write(data: [byte]);
    pub fn seek(pos: int);
    pub fn close();
}

pub class FileMetadata {
    pub v size: int;
    pub v is_dir: bool;
    pub v is_file: bool;
    pub v modified_ms: int;
}
pub fn fs_metadata(path: string) : FileMetadata;
```

Implementation note: `File` is backed by an `ELLIE_TY_HANDLE` whose drop
closes the OS handle.

### 6.3 `net` — sockets

```ei
pub class IpAddr {
    pub v host: string;     // "127.0.0.1" or "::1"
    pub v port: int;
}

// TCP
pub class TcpStream {
    co connect(host: string, port: int, timeout_ms: int);
    pub fn read(n: int) : [byte];
    pub fn read_exact(n: int) : [byte];
    pub fn write(data: [byte]) : int;
    pub fn set_read_timeout(ms: int);
    pub fn set_write_timeout(ms: int);
    pub fn local_addr() : IpAddr;
    pub fn peer_addr() : IpAddr;
    pub fn close();
}

pub class TcpListener {
    co bind(host: string, port: int);
    pub fn accept() : TcpStream;            // blocks
    pub fn local_addr() : IpAddr;
    pub fn close();
}

// UDP
pub class UdpSocket {
    co bind(host: string, port: int);
    pub fn send_to(data: [byte], host: string, port: int) : int;
    pub fn recv_from(buf_size: int) : (data: [byte], from: IpAddr);
    pub fn close();
}

// DNS
pub fn dns_lookup(host: string) : [IpAddr];
```

### 6.4 `tls` and `http`

`tls` wraps `net.TcpStream` with rustls on the native side:

```ei
pub class TlsStream {
    co connect(host: string, port: int);
    // same read/write/close surface as TcpStream
}
```

`http` is a thin client + server built on top of `net` + `tls`:

```ei
pub class HttpResponse {
    pub v status: int;
    pub v headers: [(string, string)];
    pub v body: [byte];
    pub fn text() : string;
}

pub fn http_get(url: string) : HttpResponse;
pub fn http_post(url: string, body: [byte], headers: [(string, string)]) : HttpResponse;

pub class HttpRequest {
    pub v method: string;
    pub v path: string;
    pub v headers: [(string, string)];
    pub v body: [byte];
}

// Server: blocking dispatch, one OS thread per connection by default.
pub fn http_listen(host: string, port: int, handler: fn(HttpRequest) : HttpResponse);
```

### 6.5 `process` — spawning child processes

Extends [build.ei](Ellie-Core-Library/lib/build.ei) which already has
`run_command*`. Add a proper process API.

```ei
pub class Child {
    pub v pid: int;
    pub fn stdin_write(data: [byte]);
    pub fn stdout_read(n: int) : [byte];
    pub fn stderr_read(n: int) : [byte];
    pub fn wait() : int;            // exit code, blocks
    pub fn try_wait() : NullAble<int>;
    pub fn kill();
}

pub class SpawnOptions {
    pub v cwd: string;
    pub v env: [(string, string)];
    pub v inherit_stdio: bool;
}

pub fn process_spawn(cmd: string, args: [string], opts: SpawnOptions) : Child;

pub class Output {
    pub v code: int;
    pub v stdout: string;
    pub v stderr: string;
}
pub fn process_exec(cmd: string, args: [string]) : Output;

// process / env helpers
pub fn process_args() : [string];
pub fn process_exit(code: int);
pub fn process_pid() : int;
```

### 6.6 `thread` and `channel`

Replaces the stub at [thread.ei](Ellie-Core-Library/lib/thread.ei) (currently
empty).

```ei
pub class Thread {
    pub v id: int;
    pub fn join();
    pub fn is_alive() : bool;
}
pub fn thread_spawn(f: fn()) : Thread;
pub fn thread_current_id() : int;

pub class Channel<T> {
    co bounded(capacity: int);
    pub fn send(v: T);              // blocks if full
    pub fn try_send(v: T) : bool;
    pub fn recv() : T;              // blocks if empty
    pub fn try_recv() : NullAble<T>;
    pub fn close();
}
```

The native side uses `std::thread` + `crossbeam-channel`. Ellie callbacks
across thread boundaries require care — the design is that `f: fn()` captured
state is shipped through the channel, and only "thread-safe" values (no
shared mutable heap refs) can cross. Detail TBD; mark as v1.1.

### 6.7 `crypto`, `time`, `encoding`

Smaller surfaces, ship together:

```ei
// crypto
pub fn crypto_sha256(data: [byte]) : [byte];
pub fn crypto_sha512(data: [byte]) : [byte];
pub fn crypto_random_bytes(n: int) : [byte];
pub fn crypto_random_int(min: int, max: int) : int;

// time
pub fn time_now_ms() : int;
pub fn time_now_iso() : string;          // "2026-05-21T14:33:21Z"
pub fn time_format(ms: int, fmt: string) : string;
pub fn time_parse(s: string, fmt: string) : int;

// encoding
pub fn base64_encode(data: [byte]) : string;
pub fn base64_decode(s: string) : [byte];
pub fn hex_encode(data: [byte]) : string;
pub fn hex_decode(s: string) : [byte];
pub fn json_parse(s: string) : dyn;
pub fn json_stringify(v: dyn) : string;
```

---

## 7. Raw FFI vs. native modules

Both exist and serve different jobs.

|  | Native module (L2/L3) | Raw FFI (L4) |
|---|---|---|
| Use when | You're shipping a typed, reusable component (a TCP API, a postgres driver). | You need to call a single function in a system DLL or third-party C lib. |
| Type safety | Full — Ellie type checker sees the signature. | None — you pass `(string, dyn)` arg tuples and cast the result. |
| Performance | One call → one ABI hop. | Same, plus dynamic tagging overhead per arg. |
| Distribution | Cargo-style: declare in `lia.yaml`, resolver installs. | The user supplies the .dll/.so path. |
| API | `import "tcp"` then `new TcpStream(...)`. | `load_library_handle("kernel32.dll").call("Sleep", [("uint32", 100)])` |

The raw FFI is implemented today in [run_gen2.rs](ellievm/src/run_gen2.rs)
but only for Windows and only up to 5 `u64` args. It needs:

- **libffi backend**: replace the hand-rolled `dispatch_windows_fn` with
  libffi (or dyncall). libffi handles arbitrary arity, real float/double
  passing, struct-by-value, varargs, all calling conventions.
- **Platform coverage**: SysV AMD64, AArch64 AAPCS, x86 cdecl/stdcall, etc.
  libffi covers them all.
- **Type tags**: extend the current set (`uint32 uint64 int32 int64 pointer`)
  to include `float`, `double`, `int8/16`, `uint8/16`, `cstring` (auto null
  termination), and `buffer` (raw byte array, no termination).

---

## 8. Authoring tools (`ellie native ...`)

Single CLI namespace under `elliec` (or a sibling binary, TBD).

### 8.1 `ellie native new <name> [--lang rust|c]`

Scaffolds a native library:

```
my_lib/
├── lia.yaml
├── src/
│   ├── lib.ei        # @native fn declarations + thin Ellie wrappers
│   └── lib.rs        # uses ellie_native_bridge wrapper crate
└── Cargo.toml        # cdylib, depends on ellie_native_bridge
```

For `--lang c`:

```
my_lib/
├── lia.yaml
├── src/
│   ├── lib.ei
│   ├── lib.c
│   └── ellie_bridge.h
└── CMakeLists.txt    # produces a shared library
```

### 8.2 `ellie native build [--target <triple>] [--release]`

- Resolves target triple (defaults to host).
- Runs `cargo build --release` (or `cmake --build`) under the hood.
- Emits artifact + sidecar `meta.json` (ABI version, exported symbols,
  declared capabilities) to `target/ellie-native/<triple>/`.
- Cross-compile: passes the triple through to cargo.

### 8.3 `ellie native check`

Static checks:

- `lia.yaml` is well-formed.
- Every `@native` fn declared in `exports:` files has a matching entry in
  `ellie_load_module`'s function table.
- Capability declarations match what the code actually uses (linting based on
  a deny-list — `std::process::Command` requires `process.spawn` etc.).

### 8.4 `ellie native install <path-or-url>`

Installs a built native module into `~/.ellie/cache/<triple>/`. Resolves
dependencies declared in its `lia.yaml`. Equivalent to `npm install` for
native libs.

### 8.5 `ellie native publish` (future)

Push to a registry. Out of scope for v1.

---

## 9. Memory and ownership rules

The contract that keeps native code from corrupting the VM. Rust ABI and
C ABI have different rules because they cross the boundary differently.

### 9.1 Rust ABI (primary path)

- **Strings, arrays, bytes**: passed by value. The VM allocates a `String`
  / `Vec<EllieData>` / `Vec<u8>` and hands ownership to the native fn via
  `Vec<FunctionCallParameter>`. The native fn owns the buffers from that
  point — drops them on its own schedule.
- **Return values**: `FunctionAnswer::Ok(EllieData::String(s))` transfers
  ownership of `s` back to the VM, which interns it into the heap and gives
  the Ellie program a fresh string id.
- **Cost**: one allocation each direction for non-scalar args. Acceptable
  for normal use; if a hot-path native fn cares, return `EllieData::Bytes`
  buffers instead of `Array(Vec<EllieData>)` to avoid per-element enum
  boxing.
- **No vtable** — the native side owns Rust types end-to-end. The VM does
  the conversion at the boundary, once per call.

### 9.2 C ABI (secondary path)

- `vtable->string_get(ctx, id, &len)` returns a `const char*` pointer.
  **Valid only until the next vtable call.** Native code that needs to keep
  the data must copy it.
- `vtable->string_new(ctx, data, len)` copies `data` into the VM heap and
  returns a new `string_id`. The original `data` buffer remains owned by
  the native caller.
- Strings in the VM are immutable. No `string_set`.
- `array_get` / `field_get` return values by copy; heap-typed values carry
  an id, the underlying object stays in the heap.
- `array_new` / `bytes_new` copy the input slice.
- Mutation of class fields via `field_set` is supported but discouraged —
  prefer returning new values over mutating Ellie-side state.

### 9.3 Handles (the cross-ABI bit)

`EllieData::Handle` (Rust ABI) and `ELLIE_TY_HANDLE` (C ABI) are the same
concept: an opaque native-owned resource the VM stores by reference, never
dereferences itself, and drops via a callback the native module registered.

```rust
// Rust ABI
pub struct EllieHandle {
    pub id: u64,
    // VM-internal: pointer + drop fn registered via ctx.handle_new(ptr, drop)
}

// Native fn returns a handle to a TcpStream
let sock: TcpStream = TcpStream::connect(...)?;
let handle = ctx.handle_new(Box::new(sock));   // VM stores it, returns id
FunctionAnswer::Ok(EllieData::Handle(handle))

// Later, tcp_read gets called with the handle and reads back:
let sock: &mut TcpStream = ctx.handle_get_mut(args[0].as_handle())?;
let n = sock.read(&mut buf)?;
```

Rules (apply to both ABIs):

- `drop` is called **exactly once** when the handle becomes unreachable from
  the VM.
- `drop` must be safe to call from any thread.
- `handle_get` returns `None` / `NULL` if the id is stale. Native code must
  check.
- Handles never cross module boundaries by identity — module A can't read
  module B's handles.

### 9.4 `EllieCtx`

The Rust ABI doesn't pass a ctx today — `extern "Rust" fn(args: Vec<...>)`
is the whole signature. v1.0 adds an optional ctx parameter so handle/heap
operations have something to bind to:

```rust
pub type FunctionCallback =
    extern "Rust" fn(ctx: &mut EllieCtx, arguments: Vec<FunctionCallParameter>)
        -> FunctionAnswer;
```

Old single-arg signatures are migrated by adding `_ctx: &mut EllieCtx`. The
ctx is per-call; storing it past the return of a native function is UB on
both ABIs.

### 9.5 Thread safety

- Multiple Ellie threads may call into the same native module concurrently.
  Modules must be thread-safe (interior synchronization).
- Globals in a native module are shared across all VM threads — protect
  them yourself.

---

## 10. Security and capabilities

Ellie is positioned as a language for "embedded and sandboxed environments."
Native code defeats sandboxing by default. The capability system narrows that.

### 10.1 Capability declarations

In `lia.yaml`:

```yaml
native:
  capabilities:
    - net                       # outbound network
    - net.listen                # bind/listen — separate from outbound
    - fs.read:./data            # read access, scoped path
    - fs.write:./out
    - process.spawn
    - process.env.read
    - ffi                       # raw L4 FFI to arbitrary system libs
```

### 10.2 Enforcement

- Capabilities are recorded on the module at load time (from `EllieModule`).
- The VM's runtime configuration declares which capabilities are granted to
  the running program (defaults: everything, for parity with current behavior;
  sandboxed mode: nothing unless explicitly granted).
- Mismatches are caught at module load: the loader compares declared vs.
  granted and rejects modules that ask for more than the program has.
- Inside the native code, capability checks happen at the bridge layer (e.g.
  `net.connect()` checks the `net` capability through a vtable function
  before opening a socket). Honest libraries declare; the bridge enforces.

### 10.3 Sandbox mode

`ellievm --sandbox <profile>` runs with a restricted capability set. Profiles
ship with the engine:

- `none` — pure compute, no native capabilities granted.
- `read-only` — `fs.read` on a configurable root, no network, no process.
- `network-client` — outbound `net` + `dns`, no `net.listen`, no `fs`, no
  `process`.
- `untrusted` — strictest; only stdout/stderr printing.

This intentionally mirrors Deno's permission flags more than Node's
"everything's allowed."

---

## 11. Roadmap

Concrete implementation order. Each step keeps the previous step working.

### v1.0 — Bridge cleanup (foundation)

The `ellie` binary already has a Rust-native bridge loader
([ellie/src/run_gen2.rs](ellie/src/run_gen2.rs) `load_bridge()` +
`core_native_path()`). v1.0 hardens it; no flip to C as substrate.

1. Add `abi_version: u32` to `EllieModule`, `EllieData::Bytes(Vec<u8>)`,
   `EllieData::Handle(EllieHandle)`, and `EllieModule::capabilities` in
   [EllieNativeBridge/src/rust.rs](EllieNativeBridge/src/rust.rs). All
   additive over the current shape.
2. Add the `&mut EllieCtx` parameter to `FunctionCallback` (§9.4). Wire it
   through `load_bridge` so callbacks can do `ctx.handle_new(...)` /
   `ctx.handle_get(...)`.
3. Add the version check in `load_bridge()` + clean error on mismatch.
4. Extend `raw_to_ellie_data` / `ellie_data_to_gen2` in
   [ellie/src/run_gen2.rs](ellie/src/run_gen2.rs) to handle the
   currently-silently-dropped variants (`Array`, `Class`, `Bytes`, `Handle`).
   This is where the actual conversion logic lives — no extra C hop.
5. Port [Ellie-Core-Library/native/src/lib.rs](Ellie-Core-Library/native/src/lib.rs)
   to set `abi_version` and accept `&mut EllieCtx`. Everything else stays.
6. Inline L1 fallbacks in `build_core_module()` keep working as the safety
   net for missing-cdylib scenarios, but should be kept in sync with L2.
7. **C ABI shim** — rewrite [EllieNativeBridge/src/c.rs](EllieNativeBridge/src/c.rs)
   to match §3 (vtable + `ellie_load_module` entry point). `load_bridge()`
   tries the Rust symbol first, falls back to the C symbol. Adds a single
   adapter on the engine side that maps C `EllieValue` ↔ Rust `EllieData`
   only for C-authored modules — Rust modules never touch it.
8. (Optional, same window) port `ellievm` to use the same loader so the
   split binary doesn't drift.

**Exit criteria**: `cargo test` green; existing FFI Windows test still
passes; same .ei programs run with and without the core cdylib present (L1
fallback works); a hello-world C-authored module loads alongside a Rust
core lib; ABI version mismatch produces a clear error.

### v1.1 — Std lib essentials

5. Implement `fs.*` (§6.2) on the native side, declare in `lib/fs.ei`.
6. Implement `process_spawn` / `Child` (§6.5).
7. Implement `thread_spawn` + `Channel` (§6.6).
8. Implement `time_*`, `crypto_*`, `encoding_*` (§6.7).

### v1.2 — Networking

9. `net.tcp`, `net.udp`, `dns` (§6.3).
10. `tls` via rustls (§6.4).
11. `http` client + server (§6.4).

### v1.3 — Raw FFI overhaul

12. Replace `dispatch_windows_fn` with libffi.
13. Add type tags for float/double, sub-word ints, cstring, buffer.
14. Linux + macOS coverage.

### v1.4 — Tooling

15. `ellie native new` / `build` / `check` (§8.1–8.3).
16. `lia.yaml` `native:` block parsing and resolver wiring (§4.3, §5).
17. `ellie native install` (§8.4).

### v1.5 — Sandbox

18. Capability declaration on modules.
19. Capability granting on the VM CLI (`--sandbox <profile>`).
20. Bridge-level enforcement for `net`, `fs`, `process`, `ffi`.

### Later (v2+)

- Registry + `ellie native publish`.
- Async / scheduler — would be an ABI version 2.
- WASM target (compile native modules to WASM for sandboxed embedding).
