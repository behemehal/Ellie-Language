# ellie examples

Small `.ei` programs exercising the v1.0 core native surface.

```
hello.ei         minimal — println + timestamp
hello_std.ei     same, just confirms the `import "std";` resolution works
fs_test.ei       fs_read_file / write / append / exists / dir ops / rename / remove
time_test.ei     now_ms / now_iso / format / parse / year/month/day/hour/min/sec
crypto_test.ei   sha256 + sha512 hex / base64 / hex / random_int
bytes_test.ei    bytes_alloc / get / set / slice / append + fs_read_into zero-copy
udp_test.ei      udp_bind / send_to / recv_into / last_host/port — loopback self-test
tcp_test.ei      tcp_connect / write / read — plain-HTTP GET to httpforever.com
```

## Run

From the repo root (or anywhere — the `ellie` binary finds its bundled stdlib
and core native dylib via paths next to itself):

```
ellie/target/release/ellie run ellie/examples/fs_test.ei
```

Or from the `ellie/` directory:

```
./target/release/ellie run examples/bytes_test.ei
```

## Bundled stdlib

`import "std";` resolves to the curated bundle shipped next to the `ellie`
binary at `target/<profile>/std/lib.ei`. The source-of-truth lives at
`Ellie-Core-Library/runtime/lib.ei` — copied by `ellie/build.rs` on each
release build. No local `core.ei` per project is required.

The full `Ellie-Core-Library/lib/` tree is not yet wired into the bundle
because the gen2 parser doesn't handle its full syntax (descriptions on
inner methods, tuple types in signatures, complex class bodies). Once gen2
catches up, the runtime bundle goes away and `std` resolves directly to
`Ellie-Core-Library/lib/lib.ei`.

## Known gaps

- `fs_read_dir(".")` returns 0 entries in `fs_test.ei` — `EllieData::Array`
  marshalling back into the gen2 heap isn't implemented yet, so array
  returns flatten to Void and `array.len()` then hits the unimplemented
  native and returns 0. Both gaps are pre-existing and unrelated to the new
  surface.
- `int → byte` coercion isn't supported in the gen2 type system, so byte
  arguments to `bytes_set` and the return of `bytes_get` are exposed as
  `int` for now (low 8 bits used).
- String literals don't process C-style escapes (`"a\nb"` is 4 chars).
  `bytes_from_string(s)` does process them — that's the path for binary
  protocols that need `\r\n`, `\t`, `\xNN`, etc. The HTTP request in
  `tcp_test.ei` relies on this.
- `tcp_test.ei` reaches out to `httpforever.com` over plain HTTP, so it
  needs internet. Loopback TCP self-test would require threads for the
  listener; defer until thread support lands.
