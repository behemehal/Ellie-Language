/*
MIT License
Copyright (c) 2020 Behemehal
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#ifndef _ELLIE_H
#define _ELLIE_H

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>
#include <ostream>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

enum class ArithmeticOperators {
    ARITHMETIC_OPERATORS_ADDITION,
    ARITHMETIC_OPERATORS_SUBTRACTION,
    ARITHMETIC_OPERATORS_MULTIPLICATION,
    ARITHMETIC_OPERATORS_EXPONENTIATION,
    ARITHMETIC_OPERATORS_DIVISION,
    ARITHMETIC_OPERATORS_MODULUS,
    ARITHMETIC_OPERATORS_NULL,
};

enum class ComparisonOperators {
    COMPARISON_OPERATORS_EQUAL,
    COMPARISON_OPERATORS_NOT_EQUAL,
    COMPARISON_OPERATORS_GREATER_THAN,
    COMPARISON_OPERATORS_LESS_THAN,
    COMPARISON_OPERATORS_GREATER_THAN_OR_EQUAL,
    COMPARISON_OPERATORS_LESS_THAN_OR_EQUAL,
    COMPARISON_OPERATORS_NULL,
};

enum class ConditionType {
    CONDITION_TYPE_IF,
    CONDITION_TYPE_ELSE_IF,
    CONDITION_TYPE_ELSE,
};

enum class LogicalOpearators {
    LOGICAL_OPEARATORS_AND,
    LOGICAL_OPEARATORS_OR,
    LOGICAL_OPEARATORS_NULL,
};

enum class NumberTypes {
    NUMBER_TYPES_I8,
    NUMBER_TYPES_I16,
    NUMBER_TYPES_I32,
    NUMBER_TYPES_I64,
    NUMBER_TYPES_I128,
    NUMBER_TYPES_I_SIZE,
    NUMBER_TYPES_U8,
    NUMBER_TYPES_U16,
    NUMBER_TYPES_U32,
    NUMBER_TYPES_U64,
    NUMBER_TYPES_U128,
    NUMBER_TYPES_USIZE,
    NUMBER_TYPES_F32,
    NUMBER_TYPES_F64,
};

template <typename T = void> struct Box;

struct Types;

struct NumberSize {
    enum class Tag {
        NUMBER_SIZE_U8,
        NUMBER_SIZE_U16,
        NUMBER_SIZE_U32,
        NUMBER_SIZE_U64,
        NUMBER_SIZE_U128,
        NUMBER_SIZE_USIZE,
        NUMBER_SIZE_I8,
        NUMBER_SIZE_I16,
        NUMBER_SIZE_I32,
        NUMBER_SIZE_I64,
        NUMBER_SIZE_I128,
        NUMBER_SIZE_ISIZE,
        NUMBER_SIZE_F32,
        NUMBER_SIZE_F64,
    };

    struct U8_Body {
        uint8_t _0;
    };

    struct U16_Body {
        uint16_t _0;
    };

    struct U32_Body {
        uint32_t _0;
    };

    struct U64_Body {
        uint64_t _0;
    };

    struct U128_Body {
        uintptr_t _0;
    };

    struct Usize_Body {
        uintptr_t _0;
    };

    struct I8_Body {
        int8_t _0;
    };

    struct I16_Body {
        int16_t _0;
    };

    struct I32_Body {
        int32_t _0;
    };

    struct I64_Body {
        int64_t _0;
    };

    struct I128_Body {
        intptr_t _0;
    };

    struct Isize_Body {
        intptr_t _0;
    };

    struct F32_Body {
        float _0;
    };

    struct F64_Body {
        double _0;
    };

    Tag tag;
    union {
        U8_Body U8;
        U16_Body U16;
        U32_Body U32;
        U64_Body U64;
        U128_Body U128;
        Usize_Body USIZE;
        I8_Body I8;
        I16_Body I16;
        I32_Body I32;
        I64_Body I64;
        I128_Body I128;
        Isize_Body ISIZE;
        F32_Body F32;
        F64_Body F64;
    };
};

struct NumberType {
    NumberSize value;
    const char *raw;
    NumberTypes rtype;
    bool complete;
};

struct BoolType {
    bool value;
};

struct StringType {
    const char *value;
    bool complete;
};

struct CharType {
    char value;
    bool complete;
};

struct Types;

struct RefferenceType {
    Box<Types> refference;
    bool on_dot;
    const char *const *chain;
};

struct Operators {
    enum class Tag {
        OPERATORS_COMPARISON_TYPE,
        OPERATORS_LOGICAL_TYPE,
        OPERATORS_ARITHMETIC_TYPE,
        OPERATORS_NULL,
    };

    struct ComparisonType_Body {
        ComparisonOperators _0;
    };

    struct LogicalType_Body {
        LogicalOpearators _0;
    };

    struct ArithmeticType_Body {
        ArithmeticOperators _0;
    };

    Tag tag;
    union {
        ComparisonType_Body COMPARISON_TYPE;
        LogicalType_Body LOGICAL_TYPE;
        ArithmeticType_Body ARITHMETIC_TYPE;
    };
};

struct OperatorType {
    bool cloaked;
    Box<Types> first;
    bool first_filled;
    Box<Types> second;
    bool second_is_not_null;
    Box<VariableCollector> itered_cache;
    Operators operator_;
    const char *operator_collect;
    bool operator_collected;
};

struct CloakType {
    bool complete;
    const DefinerCollecting *rtype;
    bool bracket_inserted;
    bool at_comma;
};

struct FunctionParameter {
    const char *name;
    DefinerCollecting rtype;
};

struct CursorPosition {
    int64_t _0;
    int64_t _1;
};

struct Cursor {
    CursorPosition range_start;
    CursorPosition range_end;
};

struct FunctionParameterCollector {
    FunctionParameter data;
    bool named;
    Cursor name_pos;
    bool colon_expected;
    int8_t child_brace;
    const char *type_text;
    bool typed;
    Cursor type_pos;
};

struct ArrowFunction {
    const FunctionParameterCollector *parameters;
    Box<DefinerCollecting> return_type;
    const Collecting *inside_code;
};

struct ArrowFunctionCollector {
    bool complete;
    bool param_bracket_opened;
    bool parameter_wrote;
    bool pointer_typed;
    const char *inside_code_string;
    bool return_typed;
    int64_t brace_count;
    ArrowFunction data;
};

struct FunctionCallParameter {
    Types value;
    Cursor pos;
};

struct FunctionCall {
    const char *name;
    Cursor name_pos;
    bool comma;
    bool complete;
    const FunctionCallParameter *params;
};

struct VariableType {
    bool value_complete;
    const char *value;
};

struct Types {
    enum class Tag {
        TYPES_NUMBER,
        TYPES_BOOL,
        TYPES_STRING,
        TYPES_CHAR,
        TYPES_COLLECTIVE,
        TYPES_REFFERENCE,
        TYPES_OPERATOR,
        TYPES_CLOAK,
        TYPES_ARRAY,
        TYPES_ARROW_FUNCTION,
        TYPES_FUNCTION_CALL,
        TYPES_VOID,
        TYPES_VARIABLE_TYPE,
        TYPES_NULL,
    };

    struct Number_Body {
        NumberType _0;
    };

    struct Bool_Body {
        BoolType _0;
    };

    struct String_Body {
        StringType _0;
    };

    struct Char_Body {
        CharType _0;
    };

    struct Refference_Body {
        RefferenceType _0;
    };

    struct Operator_Body {
        OperatorType _0;
    };

    struct Cloak_Body {
        CloakType _0;
    };

    struct Array_Body {
        ArrayType _0;
    };

    struct ArrowFunction_Body {
        ArrowFunctionCollector _0;
    };

    struct FunctionCall_Body {
        FunctionCall _0;
    };

    struct VariableType_Body {
        VariableType _0;
    };

    Tag tag;
    union {
        Number_Body NUMBER;
        Bool_Body BOOL;
        String_Body STRING;
        Char_Body CHAR;
        Refference_Body REFFERENCE;
        Operator_Body OPERATOR;
        Cloak_Body CLOAK;
        Array_Body ARRAY;
        ArrowFunction_Body ARROW_FUNCTION;
        FunctionCall_Body FUNCTION_CALL;
        VariableType_Body VARIABLE_TYPE;
    };
};

struct ArrayType {
    bool complete;
    Box<DefinerCollecting> rtype;
    bool bracket_inserted;
    Types len;
    bool at_comma;
    bool typed;
};

struct DynamicArrayType {
    bool complete;
    Box<DefinerCollecting> rtype;
    bool bracket_inserted;
};

struct GenericType {
    const char *rtype;
};

struct FunctionType {
    bool complete;
    const DefinerCollecting *params;
    Box<DefinerCollecting> returning;
    bool return_typed;
    int8_t return_keyword;
    bool parameter_collected;
    bool bracket_inserted;
    bool at_comma;
};

struct DefinerCollecting {
    enum class Tag {
        DEFINER_COLLECTING_ARRAY,
        DEFINER_COLLECTING_DYNAMIC_ARRAY,
        DEFINER_COLLECTING_GENERIC,
        DEFINER_COLLECTING_FUNCTION,
        DEFINER_COLLECTING_CLOAK,
        DEFINER_COLLECTING_DYNAMIC,
    };

    struct Array_Body {
        ArrayType _0;
    };

    struct DynamicArray_Body {
        DynamicArrayType _0;
    };

    struct Generic_Body {
        GenericType _0;
    };

    struct Function_Body {
        FunctionType _0;
    };

    struct Cloak_Body {
        CloakType _0;
    };

    Tag tag;
    union {
        Array_Body ARRAY;
        GrowableArray_Body DYNAMIC_ARRAY;
        Generic_Body GENERIC;
        Function_Body FUNCTION;
        Cloak_Body CLOAK;
    };
};

struct Variable {
    const char *name;
    bool dynamic;
    bool public_;
    Types value;
    Cursor pos;
};

struct VariableCollector {
    bool initialized;
    bool named;
    bool typed;
    bool value_complete;
    DefinerCollecting rtype;
    const char *raw_value;
    Variable data;
};

struct Function {
    const char *name;
    const FunctionParameterCollector *parameters;
    Types return_type;
    const Collecting *inside_code;
};

struct FunctionCollector {
    Function data;
    bool initialized;
    bool named;
    Cursor name_pos;
    bool parameter_wrote;
    Cursor parameter_bracket_start_pos;
    Cursor parameter_bracket_end_pos;
    const char *return_type_text;
    bool return_typed;
    bool pointer_typed;
    Cursor return_pointer_position;
    bool inside_object_start;
    int64_t inside_object_count;
    Cursor code_bracket_start;
    Cursor code_bracket_end;
    bool inside_code_wrote;
    const char *inside_code_string;
    bool complete;
};

struct ConditionChain {
    ConditionType rtype;
    CloakType condition;
    const Collecting *inside_code;
};

struct ConditionCollector {
    bool might_be_else_if;
    const char *else_if_keyword_collector;
    const ConditionChain *chains;
    Cursor keyword_pos;
    bool initialized;
    const char *inside_code_string;
    bool inside_object_start;
    int64_t inside_object_count;
    bool cloak_collected;
    Cursor cloak_pos;
    VariableCollector cloak_itered_data;
    bool complete;
};

struct Collecting {
    enum class Tag {
        COLLECTING_VARIABLE,
        COLLECTING_FUNCTION,
        COLLECTING_CONDITION,
        COLLECTING_NONE,
    };

    struct Variable_Body {
        VariableCollector _0;
    };

    struct Function_Body {
        FunctionCollector _0;
    };

    struct Condition_Body {
        ConditionCollector _0;
    };

    Tag tag;
    union {
        Variable_Body VARIABLE;
        Function_Body FUNCTION;
        Condition_Body CONDITION;
    };
};

struct Error {
    uint8_t code;
    const char *message;
    const char *title;
    const char *builded_message;
    const char *debug_message;
    Cursor pos;
};

struct Parsed {
    const Collecting *items;
    const Error *syntax_errors;
};

struct ParserOptions {
    bool functions;
    bool break_on_error;
    bool loops;
    bool global_variables;
    bool dynamics;
    bool collectives;
    bool variables;
};

extern "C" {

    Parsed parser_new(const char *test, ParserOptions options);

} // extern "C"

#endif // _ELLIE_H
