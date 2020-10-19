mod fifo;

use fifo::FifoScheduler;
use crate::process::thread::Thread;
use alloc::sync::Arc;

/// 线程调度器
///
/// 这里 `ThreadType` 就是 `Arc<Thread>`
pub trait Scheduler<ThreadType: Clone + Eq>: Default {
    /// 优先级的类型
    type Priority;
    /// 向线程池中添加一个线程
    fn add_thread(&mut self, thread: ThreadType);
    /// 获取下一个时间段应当执行的线程
    fn get_next(&mut self) -> Option<ThreadType>;
    /// 移除一个线程
    fn remove_thread(&mut self, thread: &ThreadType);
    /// 设置线程的优先级
    fn set_priority(&mut self, thread: ThreadType, priority: Self::Priority);
}

pub type SchedulerImpl<ThreadType> = FifoScheduler<ThreadType>;
