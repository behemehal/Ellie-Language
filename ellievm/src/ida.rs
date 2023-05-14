use ellie_engine::{
    ellie_core::defs::{DebugHeader, DebugInfo, PlatformArchitecture},
    ellie_renderer_utils::utils::{CliColor, ColorDisplay, Colors},
    ellie_vm::{
        channel::ModuleManager,
        program::{Program, VmProgram},
        thread::{Isolate, Thread},
        utils::ThreadExit,
    },
};

use crate::VmSettings;

pub fn run(program: Program, vm_settings: VmSettings, debug_file: Option<DebugInfo>) {
    let vm_program = VmProgram::new_from_vector(program.instructions);
    let mut module_manager = ModuleManager::new();

    let cli_color = &CliColor;

    let isolate = Isolate::new();
    let mut thread = Thread::new(program.main.hash, PlatformArchitecture::B64, isolate);
    thread.build_thread(program.main.clone());

}