use ellie_engine::{
    ellie_core::defs::DebugInfo,
    ellie_vm::{program::Program, thread::Thread},
};

pub struct BreakPoint {
    pub location: usize,
}

pub struct DebuggerStatus {
    pub program: Option<Program>,
    pub debug_file: Option<DebugInfo>,
    pub thread: Option<Thread>,
    pub state: DebuggerState,
    pub breakpoints: Vec<BreakPoint>,
}

pub enum DebuggerState {
    ProgramNotLoaded,
    WaitingProgram,
    WaitingAtBreakpoint,
    WaitingAtStackPoint,
    Running,
    ProgramLoaded,
    ProgramCompleted,
}
