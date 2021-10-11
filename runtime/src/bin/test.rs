use std::collections::BTreeMap;

use ellie_runtime::heap;
use ellie_runtime::stack;
use ellie_runtime::thread;

fn main() {
    let mut passed_frames = &0;
    let heap = heap::Heap::new();

    let mut stacks: BTreeMap<usize, stack::Stack> = BTreeMap::new();
    let mut main_stack = stack::Stack::new(0);
    let mut class_stack = stack::Stack::new(1);
    let mut function_stack = stack::Stack::new(2);

    main_stack.register_variable(stack::StackElement::Type(0), Some(1), false);

    stacks.insert(0, main_stack);
    stacks.insert(1, class_stack);
    stacks.insert(2, function_stack);

    let mut thread_controller = thread::ThreadController::new(
        |notify| {
            println!("THREAD FRAME: {:?}", notify);
            thread::ThreadMessage::CONTINUE
        },
        |notify| {
            println!("THREAD MESSAGE: {:?}", notify);
        },
    );

    let mut thread = thread::Thread::new(0, stacks, heap, thread_controller);
    thread.run();
}
