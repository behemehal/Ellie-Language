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

#ifndef _ELLIE_NATV_H
#define _ELLIE_NATV_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum EllieTypeDefines {
  ELLIE_TYPE_DEFINES_INT,
  ELLIE_TYPE_DEFINES_STRING,
} EllieTypeDefines;

typedef struct EllieFunctionDefineParameter {
  char *name;
  enum EllieTypeDefines data_type;
} EllieFunctionDefineParameter;

typedef enum EllieTypes_Tag {
  ELLIE_TYPES_STRING,
  ELLIE_TYPES_INT,
  ELLIE_TYPES_FLOAT,
} EllieTypes_Tag;

typedef struct EllieTypes {
  EllieTypes_Tag tag;
  union {
    struct {
      char *string;
    };
    struct {
      int int_;
    };
    struct {
      float float_;
    };
  };
} EllieTypes;

typedef struct EllieFunction {
  char *name;
  struct EllieFunctionDefineParameter *params;
  enum EllieTypeDefines returning;
  struct EllieTypes (*on_call)(struct EllieTypes*);
} EllieFunction;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct EllieFunction new_ellie_fn(char *name,
                                  struct EllieFunctionDefineParameter *params,
                                  enum EllieTypeDefines returning,
                                  struct EllieTypes (*on_call)(struct EllieTypes*));

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _ELLIE_NATV_H */
