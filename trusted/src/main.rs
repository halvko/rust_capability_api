use capability_api::object_capabilities::IO;
use capability_api::*;
fn main() {
    let io = unsafe { IO::construct() };
    let mut dc = diecup::DieCup::new(io, 5, 7);
    for _ in 0..10 {
        dc.roll();
        dc.print();
    }
}
