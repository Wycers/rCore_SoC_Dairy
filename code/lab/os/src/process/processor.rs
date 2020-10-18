/// 线程调度和管理
///
/// 休眠线程会从调度器中移除，单独保存。在它们被唤醒之前，不会被调度器安排。
pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    /// 线程调度器，记录活跃线程
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// 保存休眠线程
    sleeping_threads: HashSet<Arc<Thread>>,
}

/// 保存当前线程的 `Context`
pub fn park_current_thread(&mut self, context: &Context) {
    self.current_thread().park(*context);
}

/// 在一个时钟中断时，替换掉 context
pub fn prepare_next_thread(&mut self) -> *mut Context {
    // 向调度器询问下一个线程
    if let Some(next_thread) = self.scheduler.get_next() {
        // 准备下一个线程
        let context = next_thread.prepare();
        self.current_thread = Some(next_thread);
        context
    } else {
        // 没有活跃线程
        if self.sleeping_threads.is_empty() {
            // 也没有休眠线程，则退出
            panic!("all threads terminated, shutting down");
        } else {
            // 有休眠线程，则等待中断
            /* ... */
        }
    }
}
