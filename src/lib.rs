trait Capability {
    unsafe fn construct() -> Self;
}

pub mod object_capabilities {
    use std::{fs, io, path::Path};

    use crate::Capability;

    pub struct IO {
        _private: (),
    }

    impl Capability for IO {
        unsafe fn construct() -> Self {
            IO { _private: () }
        }
    }

    impl IO {
        pub fn write_file(
            &self,
            path: impl AsRef<Path>,
            contents: impl AsRef<[u8]>,
        ) -> io::Result<()> {
            fs::write(path, contents)
        }

        pub fn read_file(&self, path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
            fs::read(path)
        }
    }

    #[non_exhaustive]
    pub enum Operation<'call> {
        WriteFile(&'call dyn AsRef<Path>, &'call dyn AsRef<[u8]>),
        ReadFile(&'call dyn AsRef<Path>),
    }

    pub struct TempIO<F>
    where
        F: FnMut(Operation) -> bool,
    {
        io: IO,
        authorizor: F,
    }

    impl<F> TempIO<F>
    where
        F: FnMut(Operation) -> bool,
    {
        pub fn new(io: IO, authorizor: F) -> Self {
            Self { io, authorizor }
        }

        pub fn write_file(
            &mut self,
            path: impl AsRef<Path> + Clone,
            contents: impl AsRef<[u8]> + Clone,
        ) -> io::Result<()> {
            if (self.authorizor)(Operation::WriteFile(&path, &contents)) {
                return self.io.write_file(path, contents);
            }
            Result::Err(std::io::Error::new(
                io::ErrorKind::PermissionDenied,
                "The authorizer did not permit the operation",
            ))
        }

        pub fn read_file(&mut self, path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
            if (self.authorizor)(Operation::ReadFile(&path)) {
                return self.io.read_file(path);
            }
            Result::Err(std::io::Error::new(
                io::ErrorKind::PermissionDenied,
                "The authorizer did not permit the operation",
            ))
        }
    }
}

pub mod token_capabilities {
    use std::{fs, io, path};

    use crate::Capability;

    pub struct IO {
        _private: (),
    }

    impl Capability for IO {
        unsafe fn construct() -> Self {
            Self { _private: () }
        }
    }

    pub fn write_file(
        path: impl AsRef<path::Path>,
        contents: impl AsRef<[u8]>,
        _token: IO,
    ) -> io::Result<()> {
        fs::write(path, contents)
    }

    pub fn read_file(path: impl AsRef<path::Path>, _token: IO) -> io::Result<Vec<u8>> {
        fs::read(path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
