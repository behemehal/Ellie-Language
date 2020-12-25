# Ellie Engine

Low memory `no_std` engine

Adding functions to runtime


```rust

//Example print function

let dynamic_functions = vec![
runtime::DynamicFunction {
    name: "print".to_string(),
    parameters: vec![collectors::function::Parameter {
        name: "e".to_string(),
        type_of: "any".to_string()
    }],
    return_type: "void".to_string(),
    caller: |x| {
        let parameters : Vec<runtime::runtime_variable::Variable> = x.parameters;
        println!("{}", utils::print_clear(parameters[0].value.clone()));
        runtime::DynamicResponse::Void
    }
},
];

```