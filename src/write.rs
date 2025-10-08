use anyhow::{Context, Result};
use csv::Writer;
use itertools::Itertools;
use log::info;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::collatz::Unsigned;

#[derive(Serialize)]
struct Row {
    n: u64,
    cycle: String,
}

#[derive(Serialize)]
struct Cycle {
    min: String,
    count: usize,
    length: usize,
    cycle: String,
}

pub fn write_table(cycle_mins: &[Unsigned], n: u64, a: u64) -> Result<()> {
    let dir = Path::new("tables");
    fs::create_dir_all(dir).context("Failed to create 'tables' directory")?;
    let path = dir.join(format!("collatz{}.csv", a));
    let mut wtr = Writer::from_path(&path).context("Failed to create CSV writer")?;

    for x in (1..=n).step_by(2) {
        wtr.serialize(Row {
            n: x,
            cycle: cycle_mins[(x / 2) as usize].to_string(),
        })
            .context("Failed to write row to CSV")?;
    }

    wtr.flush().context("Failed to flush CSV writer")?;
    info!("Table written to {}", path.display());
    Ok(())
}

pub fn write_cycle(
    cycles: &HashMap<Unsigned, Vec<Unsigned>>,
    cycle_counts: &HashMap<&Unsigned, usize>,
    a: u64,
    p: u64
) -> Result<()> {
    let dir_name = format!("cycles_{p}");
    let dir = Path::new(&dir_name);
    fs::create_dir_all(dir).context("Failed to create 'cycles' directory")?;
    let path = dir.join(format!("cycle{}.csv", a));
    let mut wtr = Writer::from_path(&path).context("Failed to create CSV writer")?;

    for c in cycles.keys().sorted() {
        let cycle_vec = cycles.get(c).unwrap();
        let cycle_string = cycle_vec.iter().map(|u| u.to_string()).join(" -> ");
        wtr.serialize(Cycle {
            min: c.to_string(),
            count: *cycle_counts.get(c).unwrap_or(&0),
            length: cycle_vec.len(),
            cycle: cycle_string,
        })
            .context("Failed to write cycle to CSV")?;
    }

    wtr.flush().context("Failed to flush CSV writer")?;
    info!("Cycle written to {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_write_table() {
        let dir = tempdir().unwrap();
        let old_tables = std::env::current_dir().unwrap().join("tables");
        std::env::set_current_dir(dir.path()).unwrap();

        let cycle_mins = vec![
            Unsigned::from(1u64),
            Unsigned::from(5u64),
            Unsigned::from(3u64),
        ];
        write_table(&cycle_mins, 5, 3).unwrap();

        assert!(Path::new("tables/collatz3.csv").exists());

        std::env::set_current_dir(old_tables.parent().unwrap()).unwrap();
    }

    #[test]
    fn test_write_cycle() {
        let dir = tempdir().unwrap();
        let old_cycles = std::env::current_dir().unwrap().join("cycles");
        std::env::set_current_dir(dir.path()).unwrap();

        let mut cycles = HashMap::new();
        cycles.insert(
            Unsigned::from(1u64),
            vec![Unsigned::from(1u64), Unsigned::from(2u64)],
        );
        let one = Unsigned::from(1u64);
        let cycle_counts: HashMap<&Unsigned, usize> = HashMap::from([(&one, 2)]);
        write_cycle(&cycles, &cycle_counts, 3, 2).unwrap();

        assert!(Path::new("cycles/cycle3.csv").exists());

        std::env::set_current_dir(old_cycles.parent().unwrap()).unwrap();
    }
}
