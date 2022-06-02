use capabilities::{self, AuthErr, Operation};
use prelude::*;
use rand::prelude::*;

pub struct DieCup<F>
where
    F: FnMut(Operation) -> bool,
{
    dies: Vec<u8>,
    size: u8,
    io: capabilities::TempIO<F>,
}

impl<F> DieCup<F>
where
    F: FnMut(Operation) -> bool,
{
    pub fn new(io: capabilities::TempIO<F>, count: usize, size: u8) -> DieCup<F> {
        let mut rng = thread_rng();
        DieCup {
            size,
            io,
            dies: (0..count).map(|_| rng.gen_range(1..=size)).collect(),
        }
    }

    pub fn roll(&mut self) {
        let mut rng = thread_rng();
        self.dies
            .iter_mut()
            .for_each(|i| *i = rng.gen_range(1..=self.size))
    }

    pub fn print(&mut self) -> Result<(), PrintErr> {
        let str = if let Some(d) = self.dies.get(0) {
            let mut str = format!("Diecup contains: {d}");
            for e in self.dies.iter().skip(1) {
                str.push_str(&format!(", {e}"))
            }
            str.push('\n');
            str
        } else {
            format!("Diecup is empty.")
        };

        self.io.stdout(&str)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct PrintErr;

impl error::Error for PrintErr {}

impl fmt::Display for PrintErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not print diecup.")
    }
}

impl From<AuthErr> for PrintErr {
    fn from(_: AuthErr) -> Self {
        Self
    }
}
