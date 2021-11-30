Syntax:

    Function                 :   OK
    Class                    : NOT OK
    ForLoop                  :   OK
    Variable                 :   OK
    Constructor              : NOT OK
    Enums                    : NOT OK
    GetterCall               :   OK
    SetterCall               :   OK
    Getter                   : NOT OK
    Setter                   : NOT OK
    FileKey                  :   OK
    Conditions               :   OK
    Import                   :   OK
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
        Cloak                :   OK
        Array                :   OK
        Collective           :   OK
        ArrowFunction        : NOT OK  (PENDING ITEMS TO BE COMPLETE)
        VariableReference    :   OK
            SubTypes             :
                Reference        :   OK
                BraceReference   :   OK 
                Negative         :   OK
                FunctionCall     :   OK
                ClassCall        :   OK
                Logical          :   OK
                Arithmetic       :   OK
                Assignment       :   OK
                Comparison       :   OK

                34 * 100 / 40 = 85%
                Estimated complete time: December 10 2021