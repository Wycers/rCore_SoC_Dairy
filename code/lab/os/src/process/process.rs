/// 进程的信息
pub struct Process {
    /// 是否属于用户态
    pub is_user: bool,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ProcessInner>,
}

pub struct ProcessInner {
    /// 进程中的线程公用页表 / 内存映射
    pub memory_set: MemorySet,
//  /// 打开的文件描述符（实验五）
//  pub descriptors: Vec<Arc<dyn INode>>,
}
