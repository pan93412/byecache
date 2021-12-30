//! ByeCache: Modules: Base

/// The base structure of a ByeCache module.
pub trait ByeCacheMod {
    /// Execute this module.
    fn execute(self) -> Self;
    /// Wait this module to stop.
    fn wait(self) -> Self;
    /// Stop the specified handle.
    fn stop(self) -> Self;
}
