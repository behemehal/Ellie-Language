


Syntax:

    Function                    : NOT OK
    Class                       : NOT OK
    ForLoop                     : NOT OK
    Variable                    : NOT OK
    Constructor                 : NOT OK
    Enums                       : NOT OK
    Getter                      : NOT OK
    Setter                      : NOT OK
    FileKey                     : NOT OK
    Conditions                  : NOT OK
    Definers                    :   OK
        FunctionType            :   OK
        ArrayType               :   OK
        VectorType              :   OK
        CloakType               :   OK
        Collective              :   OK
        GenericType             :   OK
        NullableType            :   OK
        FutureType              :   OK
    Types                       : NOT OK
        String                  : NOT OK
        Char                    : NOT OK
        Int                     :   OK
        Float                   : NOT OK
        Cloak                   : NOT OK
        Array                   : NOT OK
        Vector                  : NOT OK
        ArrowFunction           : NOT OK
        Collective              : NOT OK
            SubTypes            : NOT OK
                Reference       : NOT OK
                BraceReference  : NOT OK 
                Negative        : NOT OK
                FunctionCall    : NOT OK
                ClassCall       : NOT OK
                Logical         : NOT OK
                Arithmetic      : NOT OK
                Assignment      : NOT OK
                Comparison      : NOT OK
