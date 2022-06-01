pub trait Capability {
    unsafe fn construct() -> Self;
}

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
