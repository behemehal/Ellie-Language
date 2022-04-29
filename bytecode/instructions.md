Auto builded from `bytecode.json` by `build.rs`
| Instruction | Implicit | Immediate | Absolute | IndirectA | IndirectB | IndirectC | IndirectX | IndirectY |
| ----------- | -------- | --------- | -------- | --------- | --------- | --------- | --------- | --------- |
| LDA         |    -    |    0x00    |   0x01   |     -     |   0x02    |   0x03    |   0x04    |   0x05    |
| LDB         |    -    |    0x06    |   0x07   |   0x08    |     -     |   0x09    |   0x0A    |   0x0B    |
| LDC         |    -    |    0x0C    |   0x0D   |   0x0E    |   0x0F    |     -     |   0x10    |   0x11    |
| LDX         |    -    |    0x12    |   0x13   |   0x14    |   0x15    |   0x16    |     -     |   0x17    |
| LDY         |    -    |    0x18    |   0x19   |   0x1A    |   0x1B    |   0x1C    |   0x1D    |     -     |
| STA         |    -    |     -      |   0x1E   |     -     |   0x20    |   0x21    |   0x22    |   0x23    |
| STB         |    -    |     -      |   0x24   |   0x25    |     -     |   0x26    |   0x27    |   0x28    |
| STC         |    -    |     -      |   0x29   |   0x2A    |   0x2B    |     -     |   0x2C    |   0x2D    |
| STX         |    -    |     -      |   0x2E   |   0x2F    |   0x30    |   0x31    |     -     |   0x32    |
| STY         |    -    |     -      |   0x33   |   0x34    |   0x35    |   0x36    |   0x37    |     -     |
| EQ          |  0x38   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| NE          |  0x39   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| GT          |  0x3A   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| LT          |  0x3B   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| GQ          |  0x3C   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| LQ          |  0x3D   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| AND         |  0x3E   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| OR          |  0x3F   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| SUB         |  0x40   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| MUL         |  0x42   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| EXP         |  0x43   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| DIV         |  0x44   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| MOD         |  0x45   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| INC         |  0x46   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| DEC         |  0x47   |     -      |    -     |     -     |     -     |     -     |     -     |     -     |
| JMP         |  0x48   |     -      |   0x49   |     -     |     -     |     -     |     -     |     -     |
| CALL        |    -    |     -      |   0x4A   |     -     |     -     |     -     |     -     |     -     |
| RET         |    -    |     -      |   0x4B   |     -     |     -     |     -     |     -     |     -     |