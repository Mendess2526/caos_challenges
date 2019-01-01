extern crate memmap;
extern crate rand;

use std::fs::File;
use memmap::MmapOptions;
use rand::Rng;
use std::time::Instant;
use std::io::StdoutLock;
use std::io::Write;


fn do_it(line :&[u8], stdout :&mut StdoutLock) -> std::io::Result<()> {
    if line.len() < 1 { return Ok(()); }
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
    stdout.write_fmt(format_args!("{}\n", sum))
}

fn main() -> std::io::Result<()> {
    let start_time = Instant::now();
    let file = File::open("input.txt")?;
    let memmap = unsafe { MmapOptions::new().map(&file)? };

    let mid_time = Instant::now();
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();
    memmap.split(|c| *c == b'\n')
        .for_each(|line| do_it(line, &mut stdout_lock).unwrap());

    let end_time = Instant::now();
    eprintln!("Loadging file: {:?}", mid_time.duration_since(start_time));
    eprintln!("Doing it:      {:?}", end_time.duration_since(mid_time));
    Ok(())
}
