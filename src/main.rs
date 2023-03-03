use std::io::{Write, BufRead, BufReader};
use std::fs::File;

type DynamicError = Box<dyn std::error::Error>;
type DynamicResult<T = ()> = Result<T, DynamicError>;

struct Prng<const M: u32, const P: u32, const I: u32> {
    state: u32,
}

impl<const M: u32, const P: u32, const I: u32> Prng<M, P, I> {
    fn new(seed: u32) -> Prng<M, P, I> {
        Prng { state: seed }
    }
}

impl<const M: u32, const P: u32, const I: u32> Iterator for Prng<M, P, I> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.state = self.state.wrapping_mul(P).wrapping_add(I) % M;

        Some(self.state)
    }
}

fn main() -> DynamicResult {
    let mut args = std::env::args().skip(1);
    let file_name = args.next().ok_or("Not enough arguments")?;
    let length: usize = args.next().ok_or("Not enough arguments")?.parse()?;
    let file = File::open(&file_name)?;
    let mut reader = BufReader::new(file);
    let mut state = String::new();

    reader.read_line(&mut state)?;
    let mut state: u32 = state.trim().parse()?;

    for n in Prng::<0x80000000, 1103515245, 12345>::new(state).take(length) {
        println!("{n}");
        state = n;
    }

    let mut file = File::create(&file_name)?;
    file.write_all(state.to_string().as_bytes())?;

    Ok(())
}
