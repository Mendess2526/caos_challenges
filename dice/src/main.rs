extern crate memmap;
extern crate memchr;
extern crate rand;

use std::fs::File;
use memmap::MmapOptions;
use memchr::Memchr;
use rand::Rng;
use std::str;


fn do_it(line :&[u8]) {
    if line.len() < 1 { return; }
    let d = Memchr::new(100, line).next().unwrap();
    let rolls = str::from_utf8(&line[0..d]).unwrap().parse::<u64>().unwrap();
    let faces = str::from_utf8(&line[(d+1)..line.len()]).unwrap().parse::<u64>().unwrap();
    let mut sum :u64 = 0;
    let mut rng = rand::thread_rng();
    for _i in 0..rolls {
        sum += 1 + rng.gen::<u64>() % faces;
    }
    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let memmap = unsafe { MmapOptions::new().map(&file)? };
    memmap.split(|c| *c == 10)
        .for_each(|line| do_it(line));
    Ok(())
}
