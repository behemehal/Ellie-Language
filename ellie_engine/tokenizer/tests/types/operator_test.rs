#[cfg(test)]
mod operator_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::processors::{
        types::{Processors, TypeProcessor},
        Processor,
    };
    const TESTS: [(&str, &str); 56] = [
        ("2 = 2 = 3", "(2 Assignment (2 Assignment 3))"),
        (" 2 + 2 == 2 + 2 ", "((2 Add 2) Equal (2 Add 2))"),
        (" 2 * 2 == 4", "((2 Mul 2) Equal 4)"),
        (" 2 / 2 == 1", "((2 Div 2) Equal 1)"),
        (" 2 % 2 == 0", "((2 Mod 2) Equal 0)"),
        (" 2 - 2 == 0", "((2 Sub 2) Equal 0)"),
        (" 2 == 2", "(2 Equal 2)"),
        (" 2 != 2", "(2 NotEqual 2)"),
        (" 2 > 2", "(2 GreaterThan 2)"),
        (" 2 >= 2", "(2 GreaterThanOrEqual 2)"),
        (" 2 < 2", "(2 LessThan 2)"),
        (" 2 <= 2", "(2 LessThanOrEqual 2)"),
        (" 2 && 2", "(2 And 2)"),
        (" 2 || 2", "(2 Or 2)"),
        (" 2 += 2", "(2 AddAssignment 2)"),
        (" 2 -= 2", "(2 SubAssignment 2)"),
        (" 2 *= 2", "(2 MulAssignment 2)"),
        (" 2 /= 2", "(2 DivAssignment 2)"),
        (" 2 %= 2", "(2 ModAssignment 2)"),
        (" 2 = 2", "(2 Assignment 2)"),
        (" 2 += 2", "(2 AddAssignment 2)"),
        (" 2 -= 2", "(2 SubAssignment 2)"),
        (" 2 *= 2", "(2 MulAssignment 2)"),
        (" 2 /= 2", "(2 DivAssignment 2)"),
        (" 2 %= 2", "(2 ModAssignment 2)"),
        (" 2 = 2", "(2 Assignment 2)"),
        (" 2 == 2", "(2 Equal 2)"),
        (" 2 != 2", "(2 NotEqual 2)"),
        (" 2 > 2", "(2 GreaterThan 2)"),
        (" 2 >= 2", "(2 GreaterThanOrEqual 2)"),
        (" 2 < 2", "(2 LessThan 2)"),
        (" 2 <= 2", "(2 LessThanOrEqual 2)"),
        (" 2 && 2", "(2 And 2)"),
        (" 2 || 2", "(2 Or 2)"),
        (" 2 += 2", "(2 AddAssignment 2)"),
        (" 2 -= 2", "(2 SubAssignment 2)"),
        (" 2 *= 2", "(2 MulAssignment 2)"),
        (" 2 /= 2", "(2 DivAssignment 2)"),
        (" 2 %= 2", "(2 ModAssignment 2)"),
        (" 2 = 2", "(2 Assignment 2)"),
        (" 2 == 2", "(2 Equal 2)"),
        (" 2 != 2", "(2 NotEqual 2)"),
        (" 2 > 2", "(2 GreaterThan 2)"),
        (" 2 >= 2", "(2 GreaterThanOrEqual 2)"),
        (" 2 < 2", "(2 LessThan 2)"),
        (" 2 <= 2", "(2 LessThanOrEqual 2)"),
        (" 2 && 2", "(2 And 2)"),
        (" 2 || 2", "(2 Or 2)"),
        (" 2 += 2", "(2 AddAssignment 2)"),
        (" 2 -= 2", "(2 SubAssignment 2)"),
        (" 2 *= 2", "(2 MulAssignment 2)"),
        (" 2 /= 2", "(2 DivAssignment 2)"),
        (" 2 %= 2", "(2 ModAssignment 2)"),
        (" 2 = 2", "(2 Assignment 2)"),
        (
            "1 == 2 && 3 == 2 + 2",
            "((1 Equal 2) And (3 Equal (2 Add 2)))",
        ),
        (
            "1 == 2 && 3 == 2 + 2 || true == true",
            "(((1 Equal 2) And (3 Equal (2 Add 2))) Or (true Equal true))",
        ),
    ];

    #[test]
    fn operator_with_no_error() {
        fn stringify_type(rtype: Processors) -> String {
            match rtype.clone() {
                Processors::Integer(e) => format!("{}", e.data.value),
                Processors::Byte(e) => format!("0x{:x}", e.value),
                Processors::Decimal(e) => match e.data.value {
                    ellie_core::definite::types::decimal::DecimalTypeEnum::Float(a) => {
                        format!("f:{}", a)
                    }
                    ellie_core::definite::types::decimal::DecimalTypeEnum::Double(a) => {
                        format!("d:{}", a)
                    }
                },
                Processors::Char(e) => format!("'{}'", e.value),
                Processors::String(e) => format!("\"{}\"", e.data.value),
                Processors::Operator(_) => stringify_opearator(rtype),
                Processors::Variable(e) => e.data.value,
                Processors::NullResolver(e) => {
                    format!("NullResolve({})", stringify_type(*e.target.clone()))
                }
                Processors::Negative(e) => format!("Neg({})", stringify_type(*e.value)),
                _ => panic!("Unexpected behaviour: {:?}", rtype),
            }
        }

        fn stringify_opearator(rtype: Processors) -> String {
            match rtype {
                Processors::Operator(e) => {
                    let first = stringify_type(*e.data.first);
                    let second = stringify_type(*e.data.second);
                    format!("({} {} {})", first, e.data.operator.to_string(), second)
                }
                _ => stringify_type(rtype),
            }
        }

        fn run_op_str(input: String) -> Result<String, String> {
            let mut pos = defs::CursorPosition::default();
            let mut errors: Vec<error::Error> = Vec::new();
            let mut processor: TypeProcessor = TypeProcessor::default();
            let mut last_char = '\0';
            for letter_char in input.chars() {
                processor.iterate(&mut errors, pos, last_char, letter_char);
                pos.skip_char(1);
                last_char = letter_char;
            }

            if errors.is_empty() {
                Ok(stringify_opearator(processor.current.clone()))
            } else {
                Err(format!("err: {:#?}", errors))
            }
        }

        for (input, expected) in TESTS.iter() {
            match run_op_str(input.to_string()) {
                Ok(result) => {
                    assert_eq!(result, expected.to_string());
                }
                Err(err) => {
                    assert!(false);
                }
            }
        }
    }
}
