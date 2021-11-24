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
    Definers                 :
        FunctionType         :   OK
        ArrayType            :   OK
        VectorType           :   OK
        CloakType            :   OK
        Collective           :   OK
        GenericType          :   OK
        NullableType         :   OK
        FutureType           :   OK
        FunctionType         :   OK
    Types                    :
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
        VariableReference    :   OK
            SubTypes             :
                Reference        :   OK
                BraceReference   : NOT OK 
                Negative         :   OK
                FunctionCall     : NOT OK
                ClassCall        : NOT OK
                Logical          :   OK
                Arithmetic       :   OK
                Assignment       :   OK
                Comparison       :   OK

                20 * 100 / 40 = 50%
                Estimated complete time: December 10 2021