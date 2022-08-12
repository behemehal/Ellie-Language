# Ellie Tools
This folder contains ellie tools. 

## Requirements
Node.js@v15>= and npm are required.


## Tools

- ### [Release](./release.js)
    Release script builds ellie binaries and makes a directory for release.

- ### [ReAssembler - TODO](./reassembler.js)
    ReAssembler is a tool for re alligning op codes or addressing modes for instructions. If a modification required in [instructions.cv](../bytecode/instructions.csv) this tool required for modifying following codes: 
    
    * HashMap at [instruction_table.rs:13](../bytecode/src/instruction_table.rs)
    * Markdown file at [instructions.md](../bytecode/instructions.md)
    * Match body at [utils.rs:178](../vm/src/utils.rs)
