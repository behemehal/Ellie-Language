# Ellie Raw Code

When compiling a file ellie first parses files in order and converts them to raw codes and feeds them to runtime.

Feeding runtime with raw code faster because there is no user input in the files. Ellie files are syntax rich and parses blank space or newlines. In the other hand raw code is pretty easy and fast to parse because positions are fixed.

Raw parser not required for now

Example ellie code to ellie raw code:

First scope: `main.ei`

```ellie


import string; //Not necessary in std ellie runtime

v test : string = "A string wow";
v aVariable : string;  


```

Import scope: `string.ei`

```ellie

pub class string {
    co string(value);
    pri v value : string;
}
```

Raw code of `string.ei`

Raw code


```ellieraw
S:
    UC 0x01
        CO 0x01 0x02
        IV 0x02 0x01

H:
    0x01 string
    0x02 value
```

Raw code of `main.ei` 


```ellieraw

import string; //Not necessary in std ellie runtime

v test : string = "A string wow";
v aVariable : string;  

#0x01 //String.ei scope
    # 0x01
    C 


S:
    PBV 0x01 0x01 0x01

H: 
    0x01 test

D: //Initial datas like d test = "test"
    0x01 "A string wow"

I:
    0x01
``` 

Scopes:


# Keys


private import         = RI
public  import         = PI

private importNative   = RN
public  importNative   = PN

private variable       = RV
public  variable       = PV

private constant       = RT
public  constant       = PT

private dynamic        = RD
public  dynamic        = PD

private function       = RF
public  function       = PF

private nativeFunction = RU
public  nativeFunction = PU

private class          = RC
public  class          = PC

private nativeClass    = RL
public  nativeClass    = PL

private enum           = RM
public  enum           = PM

        co             = CO
        if             = IF
        else if        = EF
        else           = EL
        for            = FO
        ret            = RE

        



S: //where code exist

H: //Headers like variable, class and function names

D: //Initial data like d test = "string"

Example file to raw


import ellie;

class test {
    co test();
    v test : string;
}

v var : test = new test();

-----------------

@./ellie.ei : 0x00
S: 
    PC 0x00
H:
    0x00 string
-
@./test.ei:0x00

S: 
    NC 0x01
        CO 0x01
        NV 0x01 : 




-----------------
