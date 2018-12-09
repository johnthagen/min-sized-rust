// Use the system allocator instead of bundling
// jemalloc into the binary.
use std::alloc::System;

#[global_allocator]
static A: System = System;

fn main() {
    println!("Hello, world!");
}
