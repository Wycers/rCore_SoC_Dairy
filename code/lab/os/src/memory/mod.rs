#![feature(alloc_error_handler)]

pub mod address;
pub mod frame;
pub mod mapping;
pub type MemoryResult<T> = Result<T, &'static str>;

pub mod config;
pub mod range;
mod heap;

pub fn init() {
    heap::init();
    println!("memory initialized");
}
