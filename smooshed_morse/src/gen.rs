use rand::{seq::SliceRandom, thread_rng};
use std::io::{stdout, Write};
use std::iter::once;

fn main() -> std::io::Result<()> {
    let letters = (b'a'..b'z')
        .chain(b'A'..b'B')
        .chain(once(b'\n'))
        .collect::<Vec<u8>>();
    let out = stdout();
    let mut o = out.lock();
    let mut rng = thread_rng();
    for _ in 0..10_000_000 {
        o.write(&[*letters.choose(&mut rng).unwrap()])?;
    }
    Ok(())
}
