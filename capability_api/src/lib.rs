pub trait Capability {
    unsafe fn construct() -> Self;
}

pub use std::error;
pub use std::fmt::Display;

pub mod object_capabilities;

pub mod token_capabilities;

pub mod prelude {
    pub use crate::Capability;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
