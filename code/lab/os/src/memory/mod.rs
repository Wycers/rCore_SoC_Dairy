#![feature(alloc_error_handler)]

mod heap;
mod config;

pub fn init() {
    heap::init();
    println!("memory initialized");
}
