# Developer Handbook
Welcome to ellie development handbook.

Ellie contains two main parts:
- EllieC the compiler
- Vm the virtual machine

## EllieC
EllieC is the compiler of the language. It takes a source code and generates (will) a bytecode.

EllieC has two main parts:
- Parser
- Tokenizer
- Bytecode Module

### Tokenizer

Tokenizer is the first step of the pipe line. It takes a text and returns a list of items and their positions. Major syntax errors will be reported here. You can see the raw output of the tokenizer by following command:
```
elliec tokenizer ./file.ei
```
Tokenizer has 4 options:
- `-a` `--allow-panics`: Allow tokenizer panics to be seen without covering up the error.
- `-d` `--show-debug-lines`: Shows the source of the syntax error
- `-j` `--json-log`: Outputs the tokenizer output in JSON format

### Parser

Parser is the second step of the pipe line. It takes a list of tokens and parses the meaning of the tokens. Since ellie is a type-safe language, all of the type errors will be reported here. Output of the parser is input for bytecode module. List of items are now type-safe and ready to be bytecode-compiled.

### Bytecode Module
Bytecode module is the third step of the pipe line. It takes a list of items and generates bytecode. Bytecode is the raw vm code which can be executed by the vm.

Ouputs of elliec compiler are:
- Tokenized raw code
- Parser output
- Bytecode

### Tokenized raw code

To access tokenized code you can use following command:
```
elliec tokenizer ./file.ei
```
It will generate a file called `file.json` which contains the tokenized code.

### Parser output

To access parser output you can use following command:
```
elliec compile ./file.json --output-type=bin
```
It will generate a file called `file.eic` which contains the parser output.

### Bytecode

Bytecode is the default output of the compiler. It is a raw vm code which can be executed by the vm. To access bytecode you can use following command:
```
elliec compile ./file.json --output-type=bytecode
```
It will generate a file called `file.eib` which contains the bytecode.

So, compiler generates two unviewable files: `file.eic` and `file.eib`. You can use `viewModule` command to view them.

### EllieVM (Not Implemented)
EllieVM is the univeral virtual machine. It is a virtual machine which can execute bytecode.