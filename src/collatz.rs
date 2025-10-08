use rug::{Assign, Integer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::sync::Mutex;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Unsigned {
    U64(u64),
    U128(u128),
    BigInteger(Integer),
}

impl Unsigned {
    #[inline]
    pub fn is_even(&self, p: u64) -> bool {
        match self {
            Unsigned::U64(u) => u % p == 0,
            Unsigned::U128(u) => u % (p as u128) == 0,
            Unsigned::BigInteger(i) => i.mod_u(p as u32) == 0,
        }
    }
}

impl Display for Unsigned {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Unsigned::U64(u) => write!(f, "{}", u),
            Unsigned::U128(u) => write!(f, "{}", u),
            Unsigned::BigInteger(i) => write!(f, "{}", i),
        }
    }
}

impl From<u64> for Unsigned {
    fn from(u: u64) -> Self {
        Unsigned::U64(u)
    }
}

impl From<u128> for Unsigned {
    fn from(u: u128) -> Self {
        Unsigned::U128(u)
    }
}

impl From<Integer> for Unsigned {
    fn from(i: Integer) -> Self {
        Unsigned::BigInteger(i)
    }
}

fn collatz_step(n: &mut Unsigned, a: u64, p: u64, e: u32) {
    *n = match n {
        Unsigned::U64(u) => u
            .checked_mul(a)
            .map(Unsigned::U64)
            .unwrap_or_else(|| Unsigned::U128((*u as u128) * (a as u128))),
        Unsigned::U128(u) => u
            .checked_mul(a as u128)
            .map(Unsigned::U128)
            .unwrap_or_else(|| {
                let mut i = Integer::new();
                i.assign(*u);
                Unsigned::BigInteger(i * a)
            }),
        Unsigned::BigInteger(u) => Unsigned::BigInteger(u.clone() * a),
    };

    *n = match n {
        Unsigned::U64(u) => u
            .checked_add(p.pow(e) - (*u % p.pow(e)))
            .map(Unsigned::U64)
            .unwrap_or_else(|| Unsigned::U128((*u as u128) + (p.pow(e) - (*u % p.pow(e))) as u128)),
        Unsigned::U128(u) => {
            let p128 = p as u128;
            u.checked_add(p128.pow(e) - (*u % p128.pow(e)))
                .map(Unsigned::U128)
                .unwrap_or_else(|| {
                    let mut i = Integer::new();
                    i.assign(*u);
                    Unsigned::BigInteger(i + (p128.pow(e) - (*u % p128.pow(e))) as u64)
                })
        }
        Unsigned::BigInteger(u) => Unsigned::BigInteger(u.clone() + (p.pow(e) as u32 - u.mod_u(p.pow(e) as u32))),
    };

    while n.is_even(p) {
        *n = match n {
            Unsigned::U64(u) => Unsigned::U64(*u / p),
            Unsigned::U128(u) => {
                let v = *u / (p as u128);
                u64::try_from(v)
                    .map(Unsigned::U64)
                    .unwrap_or(Unsigned::U128(v))
            }
            Unsigned::BigInteger(u) => {
                let v: Integer = u.clone() / p;
                v.to_u128()
                    .map(Unsigned::U128)
                    .unwrap_or_else(|| Unsigned::BigInteger(v))
            }
        }
    }
}

fn collatz_cycle(n: &Unsigned, a: u64, p: u64, e: u32, cycle: &mut Vec<Unsigned>) {
    let mut m = n.clone();
    while &m != n || cycle.is_empty() {
        cycle.push(m.clone());
        collatz_step(&mut m, a, p, e);
    }
    let min_id = cycle
        .iter()
        .enumerate()
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .map(|(i, _)| i)
        .unwrap();
    let mut cycle_back = cycle[..min_id].to_vec();
    *cycle = cycle[min_id..].to_owned();
    cycle.append(&mut cycle_back);
}

pub fn extended_collatz(
    n: u64,
    a: u64,
    p: u64,
    e: u32,
    cycle_mins: &Mutex<Vec<Unsigned>>,
    cycles: &Mutex<HashMap<Unsigned, Vec<Unsigned>>>,
) {
    let (mut slow, mut fast) = (Unsigned::from(n), Unsigned::from(n));
    loop {
        collatz_step(&mut slow, a, p, e);
        collatz_step(&mut fast, a, p, e);
        collatz_step(&mut fast, a, p, e);
        if slow == fast {//|| slow < un || fast < un {
            break;
        }
    }
    let cycle_min = {
        let mut cycle = Vec::new();
        collatz_cycle(&slow, a, p, e, &mut cycle);
        let cm = cycle[0].clone();

        let mut cycles_guard = cycles.lock().unwrap();
        if !cycles_guard.contains_key(&cm) {
            cycles_guard.insert(cm.clone(), cycle);
        }
        drop(cycles_guard); // Explicitly drop to release lock early
        cm
    };
    cycle_mins.lock().unwrap().push(cycle_min);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_step() {
        let mut n = Unsigned::from(3u64);
        collatz_step(&mut n, 5, 2, 1);
        assert_eq!(n, Unsigned::from(1u64));
    }

    /*#[test]
    fn test_extended_collatz() {
        let mut cycle_mins = Mutex::new(vec![Unsigned::from(1u64)]);
        let mut cycles = Mutex::new(HashMap::new());
        cycles.insert(Unsigned::from(1u64), cycle_mins.clone());
        extended_collatz(3, 3, 2, 1, &mut cycle_mins, &mut cycles);
        assert_eq!(cycle_mins.len(), 2);
        assert_eq!(cycles.len(), 1);
    }*/
}
