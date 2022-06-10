use prelude::*;
use token_capabilities::io::{print, IO};
use token_diecup as diecup;

fn main() {
    let mut print_count = 0;
    let io = unsafe { IO::construct() };
    let printer = |s: &str| {
        if print_count < 10 {
            #[allow(unused_parens)]
            {
                print!((&io), "{}", s);
            }
            print_count += 1;
            Ok(())
        } else {
            Err(())
        }
    };
    let mut dc = diecup::DieCup::new(printer, 5, 7);
    for _ in 0..10 {
        assert!(dc.print().is_ok());
        dc.roll();
    }
    assert!(dc.print().is_err());
}
