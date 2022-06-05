use prelude::*;
use token_capabilities as std;

use crate::std::io;

fn main() {
    let io = unsafe { io::IO::construct() };

    // This closure restricts io capabilities to read-only, akin to how the facet pattern does it.
    let facet = |path: &str| io::read_file(path, io);

    external(facet)
}

fn external(f: impl Fn(&str) -> io::Result<Vec<u8>>) {
    match f("/app/secret") {
        Ok(_res) => {
            // todo
        }
        Err(_) => return,
    }
}
