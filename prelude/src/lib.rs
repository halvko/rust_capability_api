pub trait Capability {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn construct() -> Self;
}

pub use std::error;
pub mod fmt {
    pub use std::fmt::Display;
}
