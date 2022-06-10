use crate::std::io::{print, IO};
use rand::prelude::*;
use token_capabilities as std;

pub struct DieCup<F, R>
where
    F: FnMut(&str) -> R,
{
    dies: Vec<u8>,
    size: u8,
    printer: F,
}

impl<F, R> DieCup<F, R>
where
    F: FnMut(&str) -> R,
{
    pub fn new(printer: F, count: usize, size: u8) -> DieCup<F, R> {
        let mut rng = thread_rng();
        DieCup {
            size,
            printer,
            dies: (0..count).map(|_| rng.gen_range(1..=size)).collect(),
        }
    }

    pub fn roll(&mut self) {
        let mut rng = thread_rng();
        self.dies
            .iter_mut()
            .for_each(|i| *i = rng.gen_range(1..=self.size))
    }

    pub fn print(&mut self) -> R {
        let str = if let Some(d) = self.dies.get(0) {
            let mut str = format!("Diecup contains: {d}");
            for e in self.dies.iter().skip(1) {
                str.push_str(&format!(", {e}"))
            }
            str.push('\n');
            str
        } else {
            "Diecup is empty.".to_string()
        };

        (self.printer)(&str)
    }
}

pub fn printer(io: IO) -> impl Fn(&str) {
    #[allow(unused_parens)]
    move |s| print!((&io), "{s}")
}
