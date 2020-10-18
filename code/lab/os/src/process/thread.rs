/// 线程的信息
pub struct Thread {
    /// 线程 ID
    pub id: ThreadID,
    /// 线程的栈
    pub stack: Range<VirtualAddress>,
    /// 所属的进程
    pub process: Arc<Process>,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ThreadInner>,
}

/// 线程中需要可变的部分
pub struct ThreadInner {
    /// 线程执行上下文
    ///
    /// 当且仅当线程被暂停执行时，`context` 为 `Some`
    pub context: Option<Context>,
    /// 是否进入休眠
    pub sleeping: bool,
    /// 是否已经结束
    pub dead: bool,
}

/// 发生时钟中断后暂停线程，保存状态
pub fn park(&self, context: Context) {
    // 检查目前线程内的 context 应当为 None
    assert!(self.inner().context.is_none());
    // 将 Context 保存到线程中
    self.inner().context.replace(context);
}

/// 准备执行一个线程
///
/// 激活对应进程的页表，并返回其 Context
pub fn prepare(&self) -> *mut Context {
    // 激活页表
    self.process.inner().memory_set.activate();
    // 取出 Context
    let parked_frame = self.inner().context.take().unwrap();
    // 将 Context 放至内核栈顶
    unsafe { KERNEL_STACK.push_context(parked_frame) }
}
