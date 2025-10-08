use anyhow::Result;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

mod collatz;
mod parse;
mod write;

use crate::collatz::extended_collatz;
use crate::parse::Args;
use crate::write::{write_cycle, write_table};

fn print_elapsed_time(start: &Instant) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!(
        "Elapsed time: {:02}:{:02}:{:02}.{:03}",
        hour,
        minute,
        second,
        millis % 1000
    );
}

fn get_exponent(a: u64, p: u64) -> u32 {
    let mut e = 1;
    let a_float = a as f64;
    let p_float = p as f64;
    while a_float.ln() - (e as f64 - 1.0 + p_float/(p_float - 1.0)) * p_float.ln() > 0.0 {
        e += 1;
    }
    e
}

fn process_collatz(
    a: u64,
    n: u64,
    p: u64,
    should_write_table: bool,
    should_write_cycle: bool,
) -> Result<()> {
    let e = get_exponent(a, p);
    let cycle_mins = Mutex::new(Vec::new());
    let cycles_mut = Mutex::new(HashMap::new());

    (1..=n)
        .filter(|x| x % p != 0)
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|x| {
            extended_collatz(x, a, p, e, &cycle_mins, &cycles_mut);
        });

    let cycles = cycles_mut.lock().unwrap();
    let cycle_mins_locked = cycle_mins.lock().unwrap();

    if should_write_table && cycles.len() > 1 {
        write_table(&cycle_mins_locked, n, a)?;
    }

    if should_write_cycle {
        let cycle_counts = cycle_mins_locked.iter().fold(HashMap::new(), |mut acc, cm| {
            *acc.entry(cm).or_insert(0) += 1;
            acc
        });
        write_cycle(&cycles, &cycle_counts, a, p)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse()?;
    let start = Instant::now();

    let results: Vec<_> = (args.a_start..=args.a_end)
        .filter(|&a| a % args.p != 0)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|a| process_collatz(a, args.n, args.p, args.write_table, args.write_cycle))
        .collect();

    // Handle errors after collecting results
    for result in results {
        result?;
    }

    print_elapsed_time(&start);
    Ok(())
}
