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

## Value Types

```rust

bool true | false
String "test"
Char "a
int 1

```

## Functions

```rust

func test(test : String) > String {
    return test;
}

v testF : func(test: String) > String = {
    return test + 1
};

```