pub mod definer_processor;
pub mod variable_processor;

pub enum Processors {
    Variable(variable_processor::VariableProcessor),
    Definer(definer_processor::DefinerProcessor),
    Null,
}
