pub mod io {

    use std::fs;

    use std::{io, path};

    pub struct IO {
        pub(crate) _private: (),
    }

    impl prelude::Capability for IO {
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

    pub fn stdin(_token: IO) -> std::io::Stdin {
        std::io::stdin()
    }
}
