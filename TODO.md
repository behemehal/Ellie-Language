# Ellie language

- Get code check syntax errors, map it save mapped file or give to compiler

# Syntax

## Variables

### Setting variable

`v variableName : variableType = variableValue;`

### Defying variable

`v variableName : variableType;`

### Setting dynamic variable

`d variableName = variableValue`

### Defying variable

`d variableName;`

```rust
v isOk : bool = false;
d isMaybeOk;

isOk = true;
isMaybeOk = false;
isMaybeOk = "no";
isOk = false;
```

```

## Functions

```rust

fn test(test : string) > string {
    return test;
}
```

## Arrays

```rust

d test = [
    1,
    2,
    3,
    [
        4,
        5
    ]
];
```
