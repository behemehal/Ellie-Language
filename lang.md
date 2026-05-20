# Ellie Language Reference

Ellie is a type-safe language designed for embedded and sandboxed environments. Source files use the `.ei` extension.

---

## Comments

```ellie
// single-line comment

/* multi-line
   comment */
```

---

## Variables & Constants

```ellie
v name = value;              // inferred type
v name : int = 5;            // explicit type
c name : int = 5;            // constant (immutable)
pub v name : int = 5;        // public
pri v name : int = 5;        // private (inside classes)
```

---

## Primitive Types

| Type     | Description                     | Example literal       |
|----------|---------------------------------|-----------------------|
| `int`    | Integer                         | `42`, `-7`            |
| `float`  | Single-precision float          | `1.3`, `.5`           |
| `double` | Double-precision float          | `3.14`                |
| `string` | UTF-8 string                    | `"hello"`             |
| `char`   | Single character                | `'a'`, `'\0'`         |
| `byte`   | Byte (hex literal)              | `0xFF`                |
| `bool`   | Boolean                         | `true`, `false`       |
| `dyn`    | Dynamic / any type              |                       |
| `void`   | No value (return type only)     |                       |

---

## Collection Types

```ellie
v arr   : [int, 5];        // static array  — 5 ints
v darr  : [int, *];        // dynamic array — any length
v cloak : (int, string);   // cloak (tuple) — ordered mixed types
v coll  : {string: int};   // collective (map) — key/value pairs
v opt   : ?int;            // nullable int
```

Literals:
```ellie
v a = [1, 2, 3];
v t = (1, "text", true);
v m = {"key": 1, "other": 2};
```

Indexing:
```ellie
v x = arr[0];
v nested = arr[arr[0]];
```

---

## Nullable Types

```ellie
v opt : ?int;
if (opt.is_null()) { ... }
v val = opt.unwrap();       // panics if null
```

---

## Function Types (as values)

```ellie
v f : @():void;             // function with no params returning void
v g : @(int, string):bool;  // function taking int+string, returning bool
v h : ?@(int):int;          // nullable function reference
```

---

## Functions

```ellie
fn greet(name: string) : string {
    ret "Hello, " + name;
}

pub fn add(a: int, b: int) : int {
    ret a + b;
}

fn nothing() {
    // no return type = void
}
```

- `ret value;` — return a value
- `ret;` — return from void function
- `pub fn` — public function

**External (native) function declaration** — no body:
```ellie
pub fn println(value: string) : void;
pub fn load_library_handle(path: string) : int;
```

---

## Generics

```ellie
class Box<T> {
    pri v value : T;
    co(value);
}

fn identity<T>(x: T) : T {
    ret x;
}

v b = new Box<int>(42);
```

---

## Classes

```ellie
class Point {
    pub v x : int;
    pub v y : int;
    co(x, y);          // shorthand constructor — assigns params to self.x, self.y
}

v p = new Point(3, 4);
```

**Constructor shorthand** `co(field1, field2)` auto-assigns constructor parameters to the matching fields.

**Custom constructor body:**
```ellie
class Library {
    co(path) {
        self.handle = load_library_handle(path);
    }
    pri v path   : string;
    pri v handle : int;
}
```

**Getters and setters:**
```ellie
class Counter {
    pri v count : int;

    pub g value() : int {
        ret self.count;
    }

    pub s value(n: int) {
        self.count = n;
    }
}
```

**Metadata decorators** (on classes or functions):
```ellie
@notConstructable = true
@dont_fix_variant = true
@native = true
@availableSince = "1.0.0"
class Internal { ... }
```

---

## Control Flow

### if / else if / else

```ellie
if (x > 0) {
    // ...
} else if (x == 0) {
    // ...
} else {
    // ...
}
```

### loop (while)

```ellie
loop condition {
    // ...
    brk;    // break
    go;     // continue
}

loop true {
    // infinite loop
}
```

### for (iteration)

```ellie
for item : collection {
    // item is the current element
}
```

---

## Operators

| Category   | Operators                              |
|------------|----------------------------------------|
| Arithmetic | `+` `-` `*` `/` `%`                   |
| Comparison | `==` `!=` `>` `>=` `<` `<=`           |
| Logical    | `&&` `\|\|` `!`                         |
| Assignment | `=` `+=` `-=` `*=` `/=` `%=`          |
| Member     | `.` (access), `[]` (index)             |
| Cast       | `as`                                   |
| Nullable   | `?` (type prefix), `.is_null()`, `.unwrap()` |

---

## Type Casting

```ellie
v s = 42 as string;
v n = "42" as int;
v t = typeof value as string;    // get runtime type name as string
```

---

## Imports

```ellie
import "./utils.ei";
pub import "./types.ei";   // re-exports the module publicly
```

Paths are relative to the importing file.

---

## Full Example — recursive fibonacci

```ellie
import "./core.ei";

fn fib(n: int) : int {
    if (n <= 1) {
        ret n;
    }
    ret fib(n - 1) + fib(n - 2);
}

fn main() {
    v result = fib(10);
    println("fib(10) = " + result);
}
```

---

## Standard Library (core.ei)

Commonly available built-ins (declared as native functions):

```ellie
println(value: string) : void
panic(message: string) : void
timestamp() : int
frame_pos() : int
```

Result/error handling pattern (from core library):
```ellie
class Error { pri v message : string; pri v code : int; co(message, code); }
class Result<T> {
    pri v value : ?T;
    pri v error : ?Error;
    co(value, error);
    fn unwrap() : string { ... }
}
```

---

## FFI / Native Interop

External functions are declared without a body. The VM resolves them at runtime:
```ellie
pub fn load_library_handle(path: string) : int;
pub fn call_library_function(handle: int, name: string, args: [(string, dyn)]) : dyn;
```

Usage:
```ellie
class Library {
    co(path) { self.handle = load_library_handle(path); }
    pri v path : string;
    pri v handle : int;
    pub fn call_function(name: string, args: [(string, dyn)]) : dyn {
        ret call_library_function(self.handle, name, args);
    }
}

fn main() {
    v lib = new Library("user32.dll");
    v result = lib.call_function("MessageBoxA", [
        ("uint64", 0), ("pointer", "Hello!"), ("pointer", "Title"), ("uint32", 0)
    ]) as int;
}
```
