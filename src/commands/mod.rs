pub mod rm;        // Declares rm.rs as a module
pub use rm::rm;    // Re-exports the rm function
pub mod  mkdir;
pub use mkdir::mkdir;
pub mod  cp;
pub use cp::cp;
// pub mod echo; in progress
pub mod cd;
pub mod pwd;
pub mod exit;
pub mod ls; //in progress
pub mod clear;
pub mod guide;
pub mod history;
pub mod cat;
pub mod mv;
