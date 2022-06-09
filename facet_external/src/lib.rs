use token_capabilities::io;

pub fn external(f: impl Fn(&str) -> io::Result<Vec<u8>>) {
    match f("/app/secret") {
        Ok(_res) => {
            // todo
        }
        Err(_) => return,
    }
}
