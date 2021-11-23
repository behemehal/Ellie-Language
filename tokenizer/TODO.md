Syntax:

    Function                 : NOT OK
    Class                    : NOT OK
    ForLoop                  : NOT OK
    Variable                 : NOT OK
    Constructor              : NOT OK
    Enums                    : NOT OK
    Getter                   : NOT OK
    Setter                   : NOT OK
    FileKey                  : NOT OK
    Conditions               : NOT OK
    Definers                 :   OK
        FunctionType         :   OK
        ArrayType            :   OK
        VectorType           :   OK
        CloakType            :   OK
        Collective           :   OK
        GenericType          :   OK
        NullableType         :   OK
        FutureType           :   OK
        FunctionType         :   OK
    Types                    : NOT OK
        String               :   OK
        Char                 :   OK
        Int                  :   OK
        Float                :   OK
        Cloak                : NOT OK
        Array                : NOT OK
        Vector               : NOT OK
        ArrowFunction        : NOT OK
        FunctionCaller       : NOT OK
        ClassConstructor     : NOT OK
        Collective           : NOT OK
        VariableReference    : NOT OK
        SubTypes             : NOT OK
            Reference        : NOT OK
            BraceReference   : NOT OK 
            Negative         : NOT OK
            FunctionCall     : NOT OK
            ClassCall        : NOT OK
            Logical          :   OK
            Arithmetic       :   OK
            Assignment       :   OK
            Comparison       :   OK

            15 * 100 / 43 = 34%
            Estimated complete time: December 10 2021