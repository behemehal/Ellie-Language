# Ellie Bytecode (Experimental)
Ellie bytecode is phase 3 of compiling process.

### Stack

#### Rules

- Moving between scopes resets active registers
- Scope id's are platform dependant in 32 bit platform its 4 bit, in 64 bit platform its 8 bit
- Absolute is platform dependant in 32 bit platform its 4 bit, in 64 bit platform its 8 bit


### Addressing Modes

| Mode      | Detail                                          |
| --------- | ----------------------------------------------- |
| implicit  | Instructions with no parameters                 |
| immediate | Instructions that takes values                  |
| absolute  | Instructions that takes stack locations         |
| indirecta | Instructions that takes A register as parameter |
| indirectb | Instructions that takes B register as parameter |
| indirectc | Instructions that takes C register as parameter |
| indirectx | Instructions that takes X register as parameter |
| indirecty | Instructions that takes Y register as parameter |
| reference | Instructions that takes reference  as parameter |


### Registers

| Register | Detail     |
| -------- | ---------- |
| A        | Accumlator | 
| B        | B Register |
| C        | B Register |
| X        | X Register |
| Y        | Y Register |


### Instructions

### Load
* LDA #VALUE
* LDB #VALUE
* LDC #VALUE
* LDX #VALUE
* LDY #VALUE
* MCA
* MCB
* MCC
* MCX
* MCY 

### Store
* STA $ADDRESS
* STB $ADDRESS
* STC $ADDRESS
* STX $ADDRESS
* STY $ADDRESS

#### Operators
* Comparison Operators
    - EQ  A == B Equal
    - NE  A != B Not equal
    - GT  A >  B Greater than
    - LT  A <  B Less than
    - GQ  A >= B Greater than or equal
    - LQ  A <= B Less than or equal
* Logical Operators
    - AND A && B Logical AND
    - OR  A || B Logical OR
* Arithmetic Operators
    - ADD B +  C = A Add
    - SUB B -  C = A Subtract
    - MUL B *  C = A Multiply
    - EXP B ^  C = A Exponent
    - DIV B /  C = A Divide
    - MOD B %  C = A Modulus
    - INC A = A + 1
    - DEC A = A - 1

### Allocation
    * 

## Example Bytecode assembly

Ellie code

```ei
v test = 1
v test2 = 2
v test3 = test + test2
```

Bytecode: 
```
LDA 1
STA #01
LDB 1
STB #02
ADD
STC #03
```

Debug Headers:
```
1: 1
2: 1
3: 2
4: 2
5: 3
6: 3
```

### [Instruction Table](./instructions.md)