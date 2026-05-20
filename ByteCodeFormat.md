# Ellie ByteCode Format

Documentation of the three output formats produced by `elliec` and how the compiler pipeline assembles them.

---

## Pipeline Overview

```
.ei source  →  Tokenizer  →  Parser  →  Assembler  →  .eic / .eig / .eia
```

The **Parser** (`ellie_parser`) produces a `Module` containing a set of `ProcessedPage`s. Each `ProcessedPage` holds resolved `Collecting` items (functions, variables, classes, imports, native functions, etc.) and a dependency graph of other pages.

The **Assembler** (`ellie_bytecode`) walks this page graph depth-first via `assemble_dependency()`. Starting from the entry module, it recursively emits instructions for each `ProcessedPage` before emitting instructions for the page that depends on it, ensuring all definitions are available before use. The result is a flat instruction list and a set of metadata structures (locals, debug headers, native call table, main function info).

Depending on the `-o` flag passed to `elliec`, the assembler renders its data into one or more output formats:

| `-o` flag      | Output files                    | Description                         |
|----------------|---------------------------------|-------------------------------------|
| `byteCode`     | `<stem>` (binary) + `<stem>.eig` | VM-executable binary + debug info   |
| `byteCodeAsm`  | `<stem>` (text)                  | Human-readable assembly listing     |

---

## `.eic` — Binary Format

The binary file has no standard file extension (the stem is used directly). It is structured as a flat byte stream:

```
┌─────────────────────────────────────────────┐
│ Header                                       │
│  [1]  arch           u8   16 / 32 / 64       │
│  [1]  has_main       u8   0 or 1             │
│  [N]  main_start     uN   LE usize (if main) │
│  [N]  main_end       uN   LE usize (if main) │
│  [N]  main_hash      uN   LE usize (if main) │
├─────────────────────────────────────────────┤
│ Native Call Table                            │
│  [N]  count          uN   LE usize           │
│  for each entry:                             │
│    [N]  mod_name_len uN   LE usize           │
│    [?]  mod_name     bytes (UTF-8)           │
│    [N]  fn_hash      uN   LE usize           │
│    [N]  fn_name_len  uN   LE usize           │
│    [?]  fn_name      bytes (UTF-8)           │
├─────────────────────────────────────────────┤
│ Instructions (concatenated op_code() bytes) │
└─────────────────────────────────────────────┘
```

Where `N` is the platform word size in bytes: 2 (B16), 4 (B32), or 8 (B64). All multi-byte integers are little-endian.

**`arch`** — target word width: `16`, `32`, or `64`.

**`has_main` / `main_start` / `main_end` / `main_hash`** — whether a `main()` function exists, its first and last instruction indices (inclusive), and its hash identifier.

**Native call table** — every `NativeFunction` declaration in the source is listed here so the VM can locate and bind the correct host function at load time. `mod_name` is the containing module's name (e.g. `"main"`, `"core"`), and `fn_name` is the declared function name.

**Instructions** — raw opcode byte streams appended one after another. See _Instruction Encoding_ below.

### Decoded example (main.ei compiled for 64-bit)

```
Offset  Bytes                            Meaning
0       64                               arch = 64-bit
1       01                               has_main = true
2-9     0A 04 00 00 00 00 00 00          main_start = 1034
10-17   88 05 00 00 00 00 00 00          main_end   = 1416
18-25   (8 bytes LE)                     main_hash
26-33   16 00 00 00 00 00 00 00          native_call_count = 22
34-41   04 00 00 00 00 00 00 00          mod_name_len = 4
42-45   6D 61 69 6E                      mod_name = "main"
46-53   (8 bytes LE)                     fn_hash (println)
54-61   07 00 00 00 00 00 00 00          fn_name_len = 7
62-68   70 72 69 6E 74 6C 6E             fn_name = "println"
...     (21 more native entries)
...     (instructions)
```

---

## Instruction Encoding

Each instruction is produced by its `op_code(arch)` method and written as a byte slice. The first byte is always the opcode. What follows depends on the addressing mode:

| Addressing Mode           | Byte layout after opcode                       |
|---------------------------|------------------------------------------------|
| `Implicit`                | *(empty — 0 extra bytes)*                      |
| `Immediate(type, bytes)`  | `[type_tag, N, 0...(N-1 zeros), ...value_bytes]` — see below |
| `Absolute(addr)`          | `[addr as N bytes LE]`                         |
| `AbsoluteIndex(ptr, idx)` | `[ptr as N bytes LE, idx as N bytes LE]`       |
| `AbsoluteProperty(p, i)`  | `[p as N bytes LE, i as N bytes LE]`           |
| `AbsoluteStatic(addr)`    | *(encodes to same bytes as Absolute)*          |
| `IndirectA/B/C/X/Y`       | *(empty — register implicit)*                  |

**Immediate layout** — type_tag identifies the primitive type (1 = int, 2 = float, 3 = double, 5 = bool, 7 = char, 10 = null, etc.). The next 8 bytes encode the data size `N` as a LE usize, then `N` value bytes follow:

```
STA #(int)1 = 47 : [1, 8, 0, 0, 0, 0, 0, 0, 0,  1, 0, 0, 0, 0, 0, 0, 0]
                    ^  ^-- size field (8 = 8 bytes) ^-- value = 1
                    type = Integer (1)
```

```
STA #(char)'s' = 47 : [7, 4, 0, 0, 0, 0, 0, 0, 0,  115, 0, 0, 0]
                       ^  ^-- size (4 bytes)          ^-- 's' = 115
                       type = Char (7)
```

```
STA #(bool)true = 47 : [5, 1, 0, 0, 0, 0, 0, 0, 0,  1]
                        ^  ^-- size (1 byte)          ^-- true = 1
                        type = Bool (5)
```

```
LDA #(null) = 1 : [10, 0, 0, 0, 0, 0, 0, 0, 0]
                   ^   ^-- size = 0 (null has no value bytes)
                   type = Null (10)
```

### Common opcodes (from the instruction table)

| Name    | Opcode | Description                                    |
|---------|--------|------------------------------------------------|
| `LDA`   | 1/2/9  | Load register A (Immediate / Absolute / @Y)    |
| `LDB`   | 11-15  | Load register B variants                       |
| `LDC`   | 20-23  | Load register C variants                       |
| `LDX`   | 28     | Load register X (Immediate)                    |
| `LDY`   | 37-42  | Load register Y variants                       |
| `STA`   | 46-48  | Store register A variants                      |
| `STB`   | 51/55  | Store register B / at index                    |
| `STC`   | 56     | Store register C                               |
| `STY`   | 66     | Store register Y                               |
| `CALL`  | 87     | Call function at absolute position             |
| `CALLN` | 116    | Call native (external) function                |
| `RET`   | 88     | Return from function                           |
| `FN`    | 118    | Function declaration marker                    |
| `SPUS`  | 96     | Stack-push (append to heap string/array)       |
| `ADD`   | 79     | Add registers B + C → A                       |
| `EQ`    | 71     | Equality comparison                            |
| `A2I`   | 104    | Convert register A to integer                  |

---

## `.eig` — Debug File Format

Produced alongside `.eic` when using `-o byteCode`. It is a plain-text file with two sections separated by `---`.

### Section 1 — Module map

One line per compiled module:

```
{module_name}E-E{module_hash}F:F {absolute_directory_path}
```

Example:
```
mainE-E4194906761F:F C:\Users\ahmet\Desktop\Ellie-Language\tools\integration_files
```

### Section 2 — Debug headers

One line per symbol (variable, function, parameter, class, etc.):

```
{instr_start}F:F{instr_end}F:F{module_path}F:F{module_hash}F:F{name}F:F{row_start}F:F{col_start}F:F{row_end}F:F{col_end}F:F{hash}{type_int}
```

Fields:
- **instr_start / instr_end** — instruction index range this symbol covers (inclusive, exclusive).
- **module_path** — source file path, `<ellie_module_main>/file.ei` for imports, bare path for the entry module.
- **module_hash** — hash of the owning module page.
- **name** — symbol name (or `@getter` / `@setter` for accessor calls).
- **row_start / col_start / row_end / col_end** — source position in the original `.ei` file.
- **hash** — unique hash for this symbol definition (used for cross-referencing with `.locals` in `.eia`).
- **type_int** — symbol kind:

| Integer | Kind            |
|---------|-----------------|
| 0       | Variable        |
| 1       | SetterCall      |
| 2       | GetterCall      |
| 3       | Class           |
| 4       | Parameter       |
| 5       | Function        |
| 6       | NativeFunction  |
| 7       | Condition       |

Example lines:
```
# Function `println` at source line 84, instructions 197-199
197F:F199F:F<ellie_module_main>/core.eiF:F1503535207F:FprintlnF:F84F:F4F:F84F:F5F:F2592667595F:F6

# Parameter `self` for a class constructor
2F:F2F:F<ellie_module_main>/core.eiF:F3435117233F:FselfF:F0F:F0F:F0F:F0F:F0F:F4

# Variable `value` spanning source lines 20-24
25F:F27F:F<ellie_module_main>/core.eiF:F1528109448F:FvalueF:F20F:F8F:F20F:F24F:F3643261392F:F0
```

---

## `.eia` — Assembly Listing Format

Produced when using `-o byteCodeAsm`. Human-readable text with four named sections.

### Section 1 — `.arch`

```
.arch {bits}
```

The target word size: `16`, `32`, or `64`.

### Section 2 — `.main`

```
.main {start}: {end} @ {hash}
```

The entry function instruction range and hash. Absent if no `main()` function was found.

### Section 3 — `.locals`

One line per local symbol (variables, functions, parameters) in instruction order:

```
{instruction_index}: {name} = {addressing_mode}[({hash})]
```

Addressing mode variants:
- `absolute` — stack-relative reference, no hash (anonymous or built-in)
- `absolute(HASH)` — stack-relative reference with symbol hash
- `absolute_static(HASH)` — page-level (static) reference with symbol hash

Examples:
```
1034: main = absolute_static(3122964542)   # main() function
1049: belirteç = absolute(634729361)       # local variable
531:  println = absolute_static(2511286163) # native function
```

### Section 4 — `.debugHeader`

Summarised form of the debug header data, one entry per symbol:

```
{TypeName} = {start}[~{end}] : {hash}
```

If `end == start + 1`, only `start` is shown; otherwise `start~end`.

```
Function = 1036~1416 : 3122964542    # main() spans instructions 1036-1416
Variable = 1049~1114 : 2169560996    # local variable
Parameter = 866~866 : 0              # unnamed parameter
NativeFunction = 197~199 : 2592667595 # println
```

### Section 5 — `.instructions`

One line per instruction:

```
{index}: {MNEMONIC} {operand} = {opcode_byte} : [{raw_bytes...}]
```

Operand notation:
- `#(type)value` — immediate value: `#(int)42`, `#(char)'A'`, `#(bool)true`, `#(null)`
- `$pos` — absolute stack position: `$531`
- `@pos[idx]` — heap dereference at position with index: `@36[0]`
- `@A` / `@B` / `@Y` — indirect through register
- *(empty)* — implicit or register-only

Examples:
```
1034: FN #(int)3122964542 = 118 : [1, 8, 0, ...]    # function declaration
1035: STA #(int)1416 = 47 : [1, 8, ...]              # store immediate int
1036: STA = 46 : []                                  # store implicit
1037: CALL $531 = 87 : [19, 2, 0, 0, 0, 0, 0, 0]   # call println at instr 531
1038: RET = 88 : []                                  # return
```

---

## Cross-Reference: `.eia` ↔ `.eig`

The `hash` values in `.locals` and `.debugHeader` correspond directly to the `hash` field in `.eig` debug header lines. The VM uses the `.eig` hash to map a runtime stack position back to a source symbol for error messages and debug output.

| `.eia` local entry              | `.eig` debug line                    |
|---------------------------------|--------------------------------------|
| `531: println = absolute_static(2592667595)` | `197F:F199F:F...F:FprintlnF:F...F:F2592667595F:F6` |

The instruction range `197~199` in `.eig` aligns with instruction index `531` in `.eia` because `.eig` uses the **debug header** instruction range (from when the symbol definition was emitted), while `.eia` uses the **cursor** (the instruction index at which the local was registered in the assembler).

---

## String Literals

Strings are assembled character-by-character. Each character is stored as a `#(char)` immediate via `STA`, then pushed onto the heap string accumulator with `SPUS`:

```eia
# Building the string "Dialog response: "
1371: STA #(char)'D' = 47 : [7, 4, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0]
1372: SPUS $1370 = 96 : [90, 5, 0, 0, 0, 0, 0, 0]
1373: STA #(char)'i' = 47 : [7, 4, 0, 0, 0, 0, 0, 0, 0, 105, 0, 0, 0]
1374: SPUS $1370 = 96 : [90, 5, 0, 0, 0, 0, 0, 0]
...
```

`SPUS $addr` appends register A's value to the heap string object at stack position `addr`.

---

## Function Call Convention

A function call sequence in the `.eia` listing follows this pattern:

```eia
# 1. Load argument into register X
LDX #(int)1369 = 28 : [...]    # store arg address in X

# 2. Call the function
CALL $531 = 87 : [19, 2, 0, 0, 0, 0, 0, 0]    # call function at instruction 531

# 3. Read return value from register Y
LDA @Y = 9 : []    # load Y (return value) into A
```

The `FN` instruction at the start of each function body records the function hash and the stack frame size. The VM uses this to set up the frame pointer when entering the function and to reclaim the frame when it returns.
