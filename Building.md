# Building And Running Ellie

Ellie's runtime is not ready but you can see parsed elements as objects.

```shell
$ cargo build --release
```

Ellie cli should be in ./target/release/ellie

```shell
$ ellie ./index.ei
```

For now you should import ellie standard library for using standard types like `string` `char` `int`

```ellie
v test : string = "test";
```

To this

```json
Variable(
    VariableCollector {
        initialized: true,
        named: true,
        typed: true,
        value_complete: false,
        raw_value: "",
        data: Variable {
            name: "test",
            dynamic: false,
            constant: false,
            public: false,
            value: VariableType(
                VariableType {
                    value_complete: false,
                    value: "test",
                    pos: Cursor {
                        range_start: CursorPosition(
                            6,
                            16,
                        ),
                        range_end: CursorPosition(
                            6,
                            19,
                        ),
                    },
                },
            ),
            pos: Cursor {
                range_start: CursorPosition(
                    6,
                    1,
                ),
                range_end: CursorPosition(
                    6,
                    20,
                ),
            },
            name_pos: Cursor {
                range_start: CursorPosition(
                    6,
                    2,
                ),
                range_end: CursorPosition(
                    6,
                    4,
                ),
            },
            value_pos: Cursor {
                range_start: CursorPosition(
                    6,
                    16,
                ),
                range_end: CursorPosition(
                    6,
                    20,
                ),
            },
            type_pos: Cursor {
                range_start: CursorPosition(
                    6,
                    7,
                ),
                range_end: CursorPosition(
                    6,
                    13,
                ),
            },
            rtype: Generic(
                GenericType {
                    rtype: "string",
                },
            ),
            hash: "",
        },
    },
)
```