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

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

typedef enum ArithmeticOperators {
  Addition,
  Subtraction,
  Multiplication,
  Exponentiation,
  Division,
  Modulus,
  Null,
} ArithmeticOperators;

typedef enum ComparisonOperators {
  Equal,
  NotEqual,
  GreaterThan,
  LessThan,
  GreaterThanOrEqual,
  LessThanOrEqual,
  Null,
} ComparisonOperators;

typedef enum ConditionType {
  If,
  ElseIf,
  Else,
} ConditionType;

typedef enum LogicalOpearators {
  And,
  Or,
  Null,
} LogicalOpearators;

typedef enum NumberTypes {
  I8,
  I16,
  I32,
  I64,
  I128,
  ISize,
  U8,
  U16,
  U32,
  U64,
  U128,
  Usize,
  F32,
  F64,
} NumberTypes;

typedef struct String String;

typedef enum NumberSize_Tag {
  U8,
  U16,
  U32,
  U64,
  U128,
  Usize,
  I8,
  I16,
  I32,
  I64,
  I128,
  Isize,
  F32,
  F64,
} NumberSize_Tag;

typedef struct NumberSize {
  NumberSize_Tag tag;
  union {
    struct {
      uint8_t u8;
    };
    struct {
      uint16_t u16;
    };
    struct {
      uint32_t u32;
    };
    struct {
      uint64_t u64;
    };
    struct {
      u128 u128;
    };
    struct {
      uintptr_t usize;
    };
    struct {
      int8_t i8;
    };
    struct {
      int16_t i16;
    };
    struct {
      int32_t i32;
    };
    struct {
      int64_t i64;
    };
    struct {
      i128 i128;
    };
    struct {
      intptr_t isize;
    };
    struct {
      float f32;
    };
    struct {
      double f64;
    };
  };
} NumberSize;

typedef struct NumberType {
  struct NumberSize value;
  const char *raw;
  enum NumberTypes r#type;
  bool complete;
} NumberType;

typedef struct BoolType {
  bool value;
} BoolType;

typedef struct StringType {
  const char *value;
  bool complete;
} StringType;

typedef struct CharType {
  char value;
  bool complete;
} CharType;

typedef struct RefferenceType {
  struct Types *refference;
  bool on_dot;
  const char *const *chain;
} RefferenceType;

typedef enum Operators_Tag {
  ComparisonType,
  LogicalType,
  ArithmeticType,
  Null,
} Operators_Tag;

typedef struct Operators {
  Operators_Tag tag;
  union {
    struct {
      enum ComparisonOperators comparison_type;
    };
    struct {
      enum LogicalOpearators logical_type;
    };
    struct {
      enum ArithmeticOperators arithmetic_type;
    };
  };
} Operators;

typedef struct OperatorType {
  bool cloaked;
  struct Types *first;
  bool first_filled;
  struct Types *second;
  bool second_is_not_null;
  struct VariableCollector *itered_cache;
  struct Operators operator_;
  const char *operator_collect;
  bool operator_collected;
} OperatorType;

typedef struct CloakType {
  bool complete;
  const struct DefinerCollecting *r#type;
  bool bracket_inserted;
  bool at_comma;
} CloakType;

typedef struct FunctionParameter {
  struct String name;
  struct DefinerCollecting r#type;
} FunctionParameter;

typedef struct CursorPosition {
  int64_t _0;
  int64_t _1;
} CursorPosition;

typedef struct Cursor {
  struct CursorPosition range_start;
  struct CursorPosition range_end;
} Cursor;

typedef struct FunctionParameterCollector {
  struct FunctionParameter data;
  bool named;
  struct Cursor name_pos;
  bool colon_expected;
  int8_t child_brace;
  const char *type_text;
  bool typed;
  struct Cursor type_pos;
} FunctionParameterCollector;

typedef struct ArrowFunction {
  const struct FunctionParameterCollector *parameters;
  struct DefinerCollecting *return_type;
  const struct Collecting *inside_code;
} ArrowFunction;

typedef struct ArrowFunctionCollector {
  bool complete;
  bool param_bracket_opened;
  bool parameter_wrote;
  bool pointer_typed;
  const char *inside_code_string;
  bool return_typed;
  int64_t brace_count;
  struct ArrowFunction data;
} ArrowFunctionCollector;

typedef struct FunctionCallParameter {
  struct Types value;
  struct Cursor pos;
} FunctionCallParameter;

typedef struct FunctionCall {
  const char *name;
  struct Cursor name_pos;
  bool comma;
  bool complete;
  const struct FunctionCallParameter *params;
} FunctionCall;

typedef struct VariableType {
  bool value_complete;
  const char *value;
} VariableType;

typedef enum Types_Tag {
  Number,
  Bool,
  String,
  Char,
  Collective,
  Refference,
  Operator,
  Cloak,
  Array,
  ArrowFunction,
  FunctionCall,
  Void,
  VariableType,
  Null,
} Types_Tag;

typedef struct Types {
  Types_Tag tag;
  union {
    struct {
      struct NumberType number;
    };
    struct {
      struct BoolType bool_;
    };
    struct {
      struct StringType string;
    };
    struct {
      struct CharType char_;
    };
    struct {
      struct RefferenceType refference;
    };
    struct {
      struct OperatorType operator_;
    };
    struct {
      struct CloakType cloak;
    };
    struct {
      struct ArrayType array;
    };
    struct {
      struct ArrowFunctionCollector arrow_function;
    };
    struct {
      struct FunctionCall function_call;
    };
    struct {
      struct VariableType variable_type;
    };
  };
} Types;

typedef struct ArrayType {
  bool complete;
  struct DefinerCollecting *r#type;
  bool bracket_inserted;
  struct Types len;
  bool at_comma;
  bool typed;
} ArrayType;

typedef struct DynamicArrayType {
  bool complete;
  struct DefinerCollecting *r#type;
  bool bracket_inserted;
} DynamicArrayType;

typedef struct GenericType {
  const char *r#type;
} GenericType;

typedef struct FunctionType {
  bool complete;
  const struct DefinerCollecting *params;
  struct DefinerCollecting *returning;
  bool return_typed;
  int8_t return_keyword;
  bool parameter_collected;
  bool bracket_inserted;
  bool at_comma;
} FunctionType;

typedef enum DefinerCollecting_Tag {
  Array,
  DynamicArray,
  Generic,
  Function,
  Cloak,
  Dynamic,
} DefinerCollecting_Tag;

typedef struct DefinerCollecting {
  DefinerCollecting_Tag tag;
  union {
    struct {
      struct ArrayType array;
    };
    struct {
      struct DynamicArrayType dynamic_array;
    };
    struct {
      struct GenericType generic;
    };
    struct {
      struct FunctionType function;
    };
    struct {
      struct CloakType cloak;
    };
  };
} DefinerCollecting;

typedef struct Variable {
  const char *name;
  bool dynamic;
  bool public_;
  struct Types value;
  struct Cursor pos;
} Variable;

typedef struct VariableCollector {
  bool initialized;
  bool named;
  bool typed;
  bool value_complete;
  struct DefinerCollecting r#type;
  const char *raw_value;
  struct Variable data;
} VariableCollector;

typedef struct Function {
  struct String name;
  const struct FunctionParameterCollector *parameters;
  struct Types return_type;
  const struct Collecting *inside_code;
} Function;

typedef struct FunctionCollector {
  struct Function data;
  bool initialized;
  bool named;
  struct Cursor name_pos;
  bool parameter_wrote;
  struct Cursor parameter_bracket_start_pos;
  struct Cursor parameter_bracket_end_pos;
  const char *return_type_text;
  bool return_typed;
  bool pointer_typed;
  struct Cursor return_pointer_position;
  bool inside_object_start;
  int64_t inside_object_count;
  struct Cursor code_bracket_start;
  struct Cursor code_bracket_end;
  bool inside_code_wrote;
  const char *inside_code_string;
  bool complete;
} FunctionCollector;

typedef struct ConditionChain {
  enum ConditionType r#type;
  struct CloakType condition;
  const struct Collecting *inside_code;
} ConditionChain;

typedef struct ConditionCollector {
  bool might_be_else_if;
  const char *else_if_keyword_collector;
  const struct ConditionChain *chains;
  struct Cursor keyword_pos;
  bool initialized;
  const char *inside_code_string;
  bool inside_object_start;
  int64_t inside_object_count;
  bool cloak_collected;
  struct Cursor cloak_pos;
  struct VariableCollector cloak_itered_data;
  bool complete;
} ConditionCollector;

typedef enum Collecting_Tag {
  Variable,
  Function,
  Condition,
  None,
} Collecting_Tag;

typedef struct Collecting {
  Collecting_Tag tag;
  union {
    struct {
      struct VariableCollector variable;
    };
    struct {
      struct FunctionCollector function;
    };
    struct {
      struct ConditionCollector condition;
    };
  };
} Collecting;

typedef struct Error {
  uint8_t code;
  const char *message;
  const char *title;
  const char *builded_message;
  const char *debug_message;
  struct Cursor pos;
} Error;

typedef struct Parsed {
  const struct Collecting *items;
  const struct Error *syntax_errors;
} Parsed;

typedef struct ParserOptions {
  bool functions;
  bool break_on_error;
  bool loops;
  bool global_variables;
  bool dynamics;
  bool collectives;
  bool variables;
} ParserOptions;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct Parsed parser_new(const char *test, struct ParserOptions options);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _ELLIE_H */
