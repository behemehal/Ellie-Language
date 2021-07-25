use alloc::vec::Vec;
use ellie_parser::syntax;

pub enum HeapWarning {
    MaxHeapScopeLength,
    MaxHeapScopeSize,
    MaxHeapSize,
}

pub struct HeapScope {
    pub scope_id: usize,
    pub variables: syntax::variable::Variable,
    pub functions: syntax::function::Function,
    pub classes: syntax::class::Class,
}

struct Heap {
    pub heaps: Vec<HeapScope>,
    pub max_heap_scope_length: usize,
    pub max_heap_scope_size: usize,
    pub max_heap_size: usize,
    pub heap_warning: fn(HeapWarning, usize),
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            heaps: Vec::new(),
            max_heap_scope_length: 5,
            max_heap_scope_size: 50,
            max_heap_size: 500,
            heap_warning: |_, _| {}
        }
    }


    pub fn create_scope() {

    }
}
