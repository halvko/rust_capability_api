use capability_api::object_capabilities as std;
use rand::prelude::*;

pub struct DieCup {
    dies: Vec<u8>,
    size: u8,
    io: std::IO,
}

impl DieCup {
    pub fn new(io: std::IO, count: usize, size: u8) -> DieCup {
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

    pub fn print(&self) {
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

        self.io.stdout(&str)
    }
}
