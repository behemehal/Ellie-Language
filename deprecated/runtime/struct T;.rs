struct T;
struct U;
struct V;

fn f(x: T, y: U) -> V {
    V
}

fn main() {
    let functions: Vec<fn(T, U) -> V> = vec![f];
}

let value = "".to_string();

if let runtime_variable::ValueTypes::TypeNumber(variable_inner_data) = variables[found_variable.unwrap()].value {
    value = variable_inner_dat a;
} else if let runtime_variable::ValueTypes::TypeString(variable_inner_data) = variables[found_variable.unwrap()].value {
    value = variable_inner_data.value;
} else if let runtime_variable::ValueTypes::TypeArray(variable_inner_data) = variables[found_variable.unwrap()].value {
    value = variable_inner_data.value;
} else if let runtime_variable::ValueTypes::TypeBool(variable_inner_data) = variables[found_variable.unwrap()].value {
    value = variable_inner_data.value;
} else if if let runtime_variable::ValueTypes::TypeFunction(variable_inner_data) = variables[found_variable.unwrap()].value {

}