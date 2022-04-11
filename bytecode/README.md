# Ellie Bytecode (Experimental)
Ellie bytecode is phase 3 of compiling process.

### Stack

#### Rules

- Moving between scopes resets active registers
- Scope id's are platform dependant in 32 bit platform its 4 bit, in 64 bit platform its 8 bit


### Addressing Modes

| Mode      | Detail                                          |
| --------- | ----------------------------------------------- |
| immediate | Instructions with no parameters                 |
| absolute  | Instructions that takes memory locations        |
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

--

```ei
v a = 1
v b = 2

fn testf(c: int, d: int) : int {
    ret c + d;
}
v c = tesf(c, d);
```

Bytecode: 
```
#00000000:
    LDA 1
    STA #01
    LDB 2
    STB #02
    CALL #01
    STC #03

#00000001:
    LDB #01
    LDC #02
    ADD
    RET $C
```

--

| Instructions | Immediate | absolute | indirecta | indirectb | indirectc | indirectx | indirecty |
| ------------ | --------- | -------- | --------- | --------- | --------- | --------- | --------- |
| LDA          | -         | 01       | 02        | 03        | 04        | 05        | 06        |
| LDB          | -         | 07       | 08        | 09        | 0A        | 0B        | 0C        |
| LDC          | -         | 0D       | 0E        | 0F        | 10        | 11        | 12        |
| LDX          | -         | 13       | 14        | 15        | 16        | 17        | 18        |
| LDY          | -         | 19       | 1A        | 1B        | 1C        | 1D        | 1E        |
| STA          | -         | 1F       | 20        | 21        | 22        | 23        | 24        |
| STB          | -         | 25       | 26        | 27        | 28        | 29        | 2A        |
| STC          | -         | 2B       | 2C        | 2D        | 2E        | 2F        | 30        |
| STX          | -         | 31       | 32        | 33        | 34        | 35        | 36        |
| STY          | -         | 37       | 38        | 39        | 3A        | 3B        | 3C        |
| EQ           | 3D        | 3F       | 40        | 41        | 42        | 43        | 44        |
| NE           | 45        | 46       | 47        | 48        | 49        | 4A        | 4B        |
| GT           | 4C        | 4D       | 4E        | 4F        | 50        | 51        | 52        |
| LT           | 53        | 54       | 55        | 56        | 57        | 58        | 59        |
| GQ           | 5A        | 5B       | 5C        | 5D        | 5E        | 5F        | 60        |
| LQ           | 61        | 62       | 63        | 64        | 65        | 66        | 67        |
| AND          | 68        | 69       | 6A        | 6A        | 6B        | 6C        | 6D        |
| OR           | 6E        | 6F       | 70        | 71        | 72        | 73        | 74        |
| ADD          | 75        | 76       | 77        | 78        | 79        | 7A        | 7B        |
| SUB          | 7C        | 7D       | 7E        | 7F        | 80        | 81        | 82        |
| MUL          | 83        | 84       | 85        | 86        | 87        | 88        | 89        |
| EXP          | 8A        | 8B       | 8C        | 8D        | 8E        | 8F        | 90        |
| DIV          | 91        | 92       | 93        | 94        | 95        | 96        | 97        |
| MOD          | 98        | 99       | 9A        | 9B        | 9C        | 9D        | 9E        |
