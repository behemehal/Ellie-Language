use crate::mapper::{defs, Collecting};
use crate::syntax::types;



#[derive(PartialEq, Debug, Clone)]
pub struct Condition {
    pub r#type: String,
    pub inside_code_string: String,
    pub inside_code: Vec<Collecting>,
    pub inside_object_start: bool,
    pub inside_object_count: i64,
    pub operator_data: types::OperatorType,
    pub complete: bool, //Fill this when end bracket placed
}

#[derive(PartialEq, Debug, Clone)]
pub struct ConditionCollector {
    pub initialized: bool,
    pub condition_chain: Vec<Condition>
}