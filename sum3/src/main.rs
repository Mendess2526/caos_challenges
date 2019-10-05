use std::collections::HashSet;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut numbers = BufReader::new(stdin().lock())
        .split(b' ')
        .map(Result::unwrap)
        .map(|s| unsafe { std::str::from_utf8_unchecked(&s).trim().parse() }.unwrap())
        .collect::<Vec<i32>>();
    numbers.sort_unstable();
    let numbers = numbers.into_boxed_slice();
    let out = stdout();
    let stdout = out.lock();
    let mut tuples = HashSet::new();
    for (i, a) in numbers[..numbers.len() - 2].iter().enumerate() {
        let mut start = i + 1;
        let mut end = numbers.len() - 1;
        while start < end {
            let b = numbers[start];
            let c = numbers[end];
            if a + b + c == 0 {
                tuples.insert((a, b, c));
                start += 1;
                end -= 1;
            } else if a + b + c > 0 {
                end -= 1;
            } else {
                start += 1;
            }
        }
    }
    let mut stdout = BufWriter::new(stdout);
    let mut tuples = tuples.iter().collect::<Vec<_>>();
    tuples.sort_unstable();
    tuples.iter().for_each(|(a, b, c)| {
        let _ = writeln!(stdout, "{} {} {}", a, b, c);
    });
}
