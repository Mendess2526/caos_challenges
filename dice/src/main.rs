extern crate memmap;
extern crate rand;

use std::fs::File;
use memmap::MmapOptions;
use rand::Rng;


fn do_it(line :&[u8]) {
    if line.len() < 1 { return; }
    let mut rolls :u64 = 0;
    let mut faces :u64 = 0;
    let mut pre_d = true;
    for c in line {
        if *c == 100 { // *c == 'd'
            pre_d = !pre_d;
            continue;
        }
        if pre_d {
            rolls = (rolls * 10) + ((*c as u64) - 48);
        }else{
            faces = (faces * 10) + ((*c as u64) - 48);
        }
    }
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
    memmap.split(|c| *c == 10) // *c == '\n'
        .for_each(|line| do_it(line));
    Ok(())
}
