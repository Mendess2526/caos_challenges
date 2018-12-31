extern crate memmap;
extern crate rand;

use std::fs::File;
use memmap::MmapOptions;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};


fn do_it(line :&[u8]) {
    if line.len() < 1 { return; }
    let mut rolls :u64 = 0;
    let mut faces :u64 = 0;
    let mut pre_d = true;
    for c in line {
        if *c == b'd' {
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
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let file = File::open("input.txt")?;
    let memmap = unsafe { MmapOptions::new().map(&file)? };

    let mid_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    memmap.split(|c| *c == b'\n') // *c == '\n'
        .for_each(|line| do_it(line));

    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let load_time = mid_time - start_time;
    let do_time   = end_time - mid_time;
    eprintln!("Loadging file: {} micros", load_time.as_secs() * 1000 + load_time.subsec_nanos() as u64 / 1_000);
    eprintln!("Doing it:      {} micros", do_time.as_secs()   * 1000 + do_time.subsec_nanos()   as u64 / 1_000);
    Ok(())
}
