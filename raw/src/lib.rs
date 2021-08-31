pub mod converter;
pub mod parser;

extern crate alloc;

#[derive(Clone, Debug)]
pub enum Commands {
    RI, //Private import
    PI, //Public import
    RN, //Private native import
    PN, //Public native import
    RV, //Private variable
    PV, //Public variable
    RT, //Private constant
    PT, //Public constant
    RD, //Private dynamic
    PD, //Public dynamic
    RF, //Private function
    PF, //Public function
    RU, //Private native function
    PU, //Public native function
    RC, //Private class
    PC, //Public class
    RL, //Private native class
    PL, //Public native class
    IF, //If
    EF, //Else if
    EL, //Else
    FO, //For
    RE  //Ret
}

impl Default for Commands {
    fn default() -> Self {
        Commands::PI
    }
}


#[derive(Default, Clone, Debug)]
pub struct Item {
    pub rtype: Commands,
    pub name: usize,
    pub has_params: bool,
    pub params: Vec<(usize, String)>,
    pub has_generics: bool,
    pub generics: Vec<String>,
    pub has_type: bool,
    pub type_id: isize,
    pub has_initial: bool,
    pub initial_data: String,
    pub has_inner_scope: bool,
    pub inner_scope_name: usize
}


