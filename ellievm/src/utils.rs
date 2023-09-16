use ellie_engine::{
    ellie_core::defs::DebugInfo,
    ellie_vm::{
        channel::ModuleManager,
        program::{Program, VmProgram},
        thread::Thread,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct BreakPoint {
    pub module_name: Option<String>,
    pub stack_location: usize,
    pub code_location: Option<usize>,
}

pub struct DebuggerStatus<'a> {
    pub vm_program: Option<VmProgram>,
    pub program: Option<Program>,
    pub debug_file: Option<DebugInfo>,
    pub thread: &'a mut Thread,
    pub state: DebuggerState,
    pub breakpoints: Vec<BreakPoint>,
    pub module_manager: ModuleManager,
}

#[derive(PartialEq, Debug)]
pub enum DebuggerState {
    ProgramNotLoaded,
    WaitingAtBreakpoint(BreakPoint),
    Running,
    ProgramLoaded,
    ProgramCompleted,
}

impl DebuggerState {
    pub fn to_string(&self) -> &'static str {
        match self {
            DebuggerState::ProgramNotLoaded => "ProgramNotLoaded",
            DebuggerState::WaitingAtBreakpoint(bp) => "WaitingAtBreakpoint",
            DebuggerState::Running => "Running",
            DebuggerState::ProgramLoaded => "ProgramLoaded",
            DebuggerState::ProgramCompleted => "ProgramCompleted",
        }
    }
}
