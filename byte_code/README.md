# Ellie Byte Code [EXPERIMENTAL]

When compiling a file ellie first parses files in order and converts them to byte codes and feeds them to runtime.

Feeding runtime with raw code faster because there is no user input in the files. Ellie files are syntax rich and parses blank space or newlines. In the other hand raw code is pretty easy and fast to parse because positions are fixed.

Example Conversion:

```ellie

v test : string = 123;

```

```

@0x01
    S:
        PC 0x00

    H:
        0x00 int


@0x00
    S:
        RV 0x00 : 0x00 = 0x01
    H:
        0x00 test
    I:
        0x00 123

```