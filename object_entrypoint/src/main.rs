use object_capabilities::{Operation, TempIO, IO};
use prelude::*;
fn main() {
    let mut print_count = 0;
    let io = TempIO::new(unsafe { IO::construct() }, |o| match o {
        Operation::StdOut(_) => {
            if print_count < 10 {
                print_count += 1;
                true
            } else {
                false
            }
        }
        _ => false,
    });
    let mut dc = diecup::DieCup::new(io, 5, 7);
    for _ in 0..10 {
        assert!(dc.print().is_ok());
        dc.roll();
    }
    assert!(dc.print().is_err());
}
