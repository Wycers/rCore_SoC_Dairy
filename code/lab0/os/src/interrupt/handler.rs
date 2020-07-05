use super::context::Context;
use riscv::register::stvec;
use riscv::register::scause::{Scause, Trap, Exception, Interrupt};
use crate::interrupt::timer;

global_asm!(include_str!("../asm/interrupt.asm"));

/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
        }
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

/// 中断的处理入口
///
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(_context: &mut Context, scause: Scause, _stval: usize) {
    // match scause.cause() {
    //     Trap::Exception(Exception::Breakpoint) => breakpoint(context),
    //     Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
    //     _ => unimplemented!("{:?}: {:x?}, stval: 0x{:x}", scause.cause(), context, stval),
    // }
    panic!("Interrupted: {:?}", scause.cause());
}

fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    for (index, reg_val) in context.x[1..].iter().enumerate() {
        println!("Value of register x{} is {:x}", index + 1, reg_val);
    }
    context.sepc += 2;
}

fn supervisor_timer(_: &Context) {
    timer::tick();
}
