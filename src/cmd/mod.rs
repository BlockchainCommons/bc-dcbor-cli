use anyhow::Result;

pub mod array;
pub mod default;
pub mod map;
pub mod r#match;

/// Trait for command execution
pub trait Exec {
    fn exec(&self) -> Result<String>;
}
