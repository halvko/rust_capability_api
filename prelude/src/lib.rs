pub trait Capability {
    unsafe fn construct() -> Self;
}

pub use std::error;
pub mod fmt {
    pub use std::fmt::Display;
}
