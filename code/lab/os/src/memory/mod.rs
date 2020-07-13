#![feature(alloc_error_handler)]

mod address;
pub mod config;
mod heap;

pub fn init() {
    heap::init();
    println!("memory initialized");
}
