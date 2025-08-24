pub mod rm;        // Declares rm.rs as a module
pub use rm::rm;    // Re-exports the rm function
pub mod  mkdir;
pub use mkdir::mkdir;
pub mod  cp;
pub use cp::cp;