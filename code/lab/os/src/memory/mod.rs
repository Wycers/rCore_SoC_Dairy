#![feature(alloc_error_handler)]

pub mod address;
pub mod frame;
pub mod mapping;
pub type MemoryResult<T> = Result<T, &'static str>;

mod config;
mod heap;
pub mod range;

pub fn init() {
    heap::init();
    println!("memory initialized");
}
