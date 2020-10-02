#![feature(alloc_error_handler)]

pub mod address;
pub mod frame;
pub type MemoryResult<T> = Result<T, &'static str>;

mod config;
mod heap;
mod range;


pub fn init() {
    heap::init();
    println!("memory initialized");
}
