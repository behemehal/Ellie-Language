#ifndef ellie
#define ellie

struct ParserOptions {
    bool functions;
    bool break_on_error;
    bool loops;
    bool global_variables;
    bool dynamics;
    bool collectives;
    bool variables;
};

typedef enum {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
} Colors;

char* get_color(Colors);

#endif