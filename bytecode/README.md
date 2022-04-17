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
| LDA          | --        | 02       | 03        | 04        | 05        | 05        | 06        |
| LDB          | 07 (I)    | 08       | 09        | 0A        | 0B        | 0C        | 0D        |
| LDC          | 0E (I)    | 0F       | 10        | 11        | 12        | 13        | 14        |
| LDX          | 15 (I)    | 16       | 17        | 18        | 19        | 1A        | 1B        |
| LDY          | 1C (I)    | 1D       | 1E        | 1F        | 20        | 21        | 22        |
| STA          | 23        | 24       | 25        | 26        | 27        | 28        | 29        |
| STB          | 2A        | 2B       | 2C        | 2D        | 2E        | 2F        | 30        |
| STC          | 31        | 32       | 33        | 34        | 35        | 36        | 37        |
| STX          | 38        | 39       | 3A        | 3B        | 3C        | 3D        | 3E        |
| STY          | 3F        | 40       | 41        | 42        | 43        | 44        | 45        |
| EQ           | 46        | 47       | 48        | 49        | 4A        | 4B        | 4C        |
| NE           | 4D        | 4E       | 4F        | 50        | 51        | 52        | 53        |
| GT           | 54        | 55       | 56        | 57        | 58        | 59        | 5A        |
| LT           | 5B        | 5C       | 5D        | 5E        | 5F        | 60        | 61        |
| GQ           | 62        | 63       | 64        | 65        | 66        | 67        | 68        |
| LQ           | 69        | 6A       | 6B        | 6C        | 6D        | 6E        | 6F        |
| AND          | 70        | 71       | 72        | 73        | 74        | 75        | 76        |
| OR           | 77        | 78       | 79        | 7A        | 7B        | 7C        | 7D        |
| ADD          | 7E        | 7F       | 80        | 81        | 82        | 83        | 84        |
| SUB          | 85        | 86       | 87        | 88        | 89        | 8A        | 8B        |
| MUL          | 8C        | 8D       | 8E        | 8F        | 90        | 91        | 92        |
| EXP          | 93        | 94       | 95        | 96        | 97        | 98        | 99        |
| DIV          | 9A        | 9B       | 9C        | 9D        | 9E        | 9F        | A0        |
| MOD          | A1        | A2       | A3        | A4        | A5        | A6        | A7        |
| INC          | A8        | A9       | AA        | AB        | AC        | AD        | AE        |
| DEC          | AF        | B0       | B1        | B2        | B3        | B4        | B5        |
| CALL         | B6        | B7       | B8        | B9        | BA        | BB        | BC        |
| RET          | BD        | BE       | BF        | C0        | C1        | C2        | C3        |
| JMP          | C4 (I)    | C5       | C6 (I)    | C7 (I)    | C8 (I)    | C9 (I)    | CA (I)    |
| DRP          | --        | 01       | --        | --        | --        | --        | --        |

(I): Illegal instruction
