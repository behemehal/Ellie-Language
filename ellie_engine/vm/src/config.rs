// VM's stack memory size
// 24 * 1024 = 24kb
pub const STACK_MEMORY_SIZE: usize = 4048 + 1024;

// VM's stack size
// 168 * 512 = 86kb
pub const STACK_SIZE: usize = 256;

//Program size
// ~48 bytes * 8192 = ~384kb
pub const PROGRAM_MAX_SIZE: usize = 8192;
