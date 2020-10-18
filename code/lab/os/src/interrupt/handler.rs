use super::context::Context;
use crate::interrupt::timer;
use riscv::register::stvec;
use riscv::register::scause::{Scause, Trap, Exception, Interrupt};

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
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    // 可以通过 Debug 来查看发生了什么中断
    // println!("{:x?}", context.scause.cause());
    // match scause.cause() {
    //     // 断点中断（ebreak）
    //     Trap::Exception(Exception::Breakpoint) => breakpoint(context),
    //     // 时钟中断
    //     Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
    //     // 其他情况，终止当前线程
    //     _ => fault(context, scause, stval),
    // }
    panic!("Interrupted: {:?}", scause.cause());
}

fn breakpoint(context: &mut Context) -> *mut Context {
    println!("Breakpoint at 0x{:x}", context.sepc);
    for (index, reg_val) in context.x[1..].iter().enumerate() {
        println!("Value of register x{} is {:x}", index + 1, reg_val);
    }
    context.sepc += 2;
    context
}

/// 处理时钟中断
fn supervisor_timer(_: &Context) -> *mut Context {
    timer::tick();
    PROCESSOR.lock().park_current_thread(context);
    PROCESSOR.lock().prepare_next_thread()
}
