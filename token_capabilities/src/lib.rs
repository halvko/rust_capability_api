pub mod io {

    use std::fs;

    use std::{io, path};

    pub use io::Result;

    #[derive(Copy, Clone)]
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

    #[macro_export]
    macro_rules! println {
        ($cap:tt) => {{
           fn c(_: $crate::io::IO){}
            c($cap);
            std::println!()
        }};
        ($cap:tt, $($arg:tt)*) => {{
           fn c(_: $crate::io::IO){}
           c($cap);
           std::println!($($arg)*)
        }}
    }
    pub use crate::println;

    #[macro_export]
    macro_rules! print {
        ($cap:tt) => {{
           fn c(_: $crate::io::IO){}
            c($cap);
            std::print!()
        }};
        ($cap:tt, $($arg:tt)*) => {{
           fn c(_: $crate::io::IO){}
           c($cap);
           std::print!($($arg)*)
        }}
    }
    pub use crate::print;
}
