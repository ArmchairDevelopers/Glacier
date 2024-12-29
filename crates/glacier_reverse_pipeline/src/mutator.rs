pub trait PipelineMutator {
    /// Whether imports in assets should be resolved.
    fn allow_shallow_load(&self) -> bool {
        false
    }
}
