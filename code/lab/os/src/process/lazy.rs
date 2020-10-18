lazy_static! {
    /// 全局的 [`Processor`]
    pub static ref PROCESSOR: Lock<Processor> = Lock::new(Processor::default());
}
