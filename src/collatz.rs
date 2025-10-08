use rug::{Assign, Integer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Unsigned {
    U64(u64),
    U128(u128),
    BigInteger(Integer),
}

impl Unsigned {
    #[inline]
    pub fn is_even(&self) -> bool {
        match self {
            Unsigned::U64(u) => u % 2 == 0,
            Unsigned::U128(u) => u % 2 == 0,
            Unsigned::BigInteger(i) => i.is_even(),
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

fn collatz_step(n: &mut Unsigned, a: u64, p: u64) {
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
            .checked_add(p - (*u & (p - 1)))
            .map(Unsigned::U64)
            .unwrap_or_else(|| Unsigned::U128((*u as u128) + (p - (*u & (p - 1))) as u128)),
        Unsigned::U128(u) => {
            let p128 = p as u128;
            u.checked_add(p128 - (*u & (p128 - 1)))
                .map(Unsigned::U128)
                .unwrap_or_else(|| {
                    let mut i = Integer::new();
                    i.assign(*u);
                    Unsigned::BigInteger(i + (p128 - (*u & (p128 - 1))) as u64)
                })
        }
        Unsigned::BigInteger(u) => Unsigned::BigInteger(u.clone() + (p as u32) - u.mod_u(p as u32)),
    };

    while n.is_even() {
        *n = match n {
            Unsigned::U64(u) => Unsigned::U64(*u / 2),
            Unsigned::U128(u) => {
                let v = *u / 2;
                u64::try_from(v)
                    .map(Unsigned::U64)
                    .unwrap_or(Unsigned::U128(v))
            }
            Unsigned::BigInteger(u) => {
                let v: Integer = u.clone() / 2;
                v.to_u128()
                    .map(Unsigned::U128)
                    .unwrap_or_else(|| Unsigned::BigInteger(v))
            }
        }
    }
}

fn collatz_cycle(n: &Unsigned, a: u64, p: u64, cycle: &mut Vec<Unsigned>) {
    let mut m = n.clone();
    while &m != n || cycle.is_empty() {
        cycle.push(m.clone());
        collatz_step(&mut m, a, p);
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
    cycle_mins: &mut Vec<Unsigned>,
    cycles: &mut HashMap<Unsigned, Vec<Unsigned>>,
) {
    let (mut slow, mut fast, un) = (Unsigned::from(n), Unsigned::from(n), Unsigned::from(n));
    loop {
        collatz_step(&mut slow, a, p);
        collatz_step(&mut fast, a, p);
        collatz_step(&mut fast, a, p);
        if slow == fast || slow < un || fast < un {
            break;
        }
    }
    let cycle_min = if slow < un {
        if let Unsigned::U64(m) = slow {
            cycle_mins[(m / 2) as usize].clone()
        } else {
            panic!("Unexpected Unsigned variant")
        }
    } else if fast < un {
        if let Unsigned::U64(m) = fast {
            cycle_mins[(m / 2) as usize].clone()
        } else {
            panic!("Unexpected Unsigned variant")
        }
    } else {
        let mut cycle = Vec::new();
        collatz_cycle(&slow, a, p, &mut cycle);
        let cm = cycle[0].clone();
        if !cycles.contains_key(&cm) {
            cycles.insert(cm.clone(), cycle);
        }
        cm
    };
    cycle_mins.push(cycle_min);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_step() {
        let mut n = Unsigned::from(3u64);
        collatz_step(&mut n, 5, 4);
        assert_eq!(n, Unsigned::from(1u64));
    }

    #[test]
    fn test_extended_collatz() {
        let mut cycle_mins = vec![Unsigned::from(1u64)];
        let mut cycles = HashMap::new();
        cycles.insert(Unsigned::from(1u64), cycle_mins.clone());
        extended_collatz(3, 3, 2, &mut cycle_mins, &mut cycles);
        assert_eq!(cycle_mins.len(), 2);
        assert_eq!(cycles.len(), 1);
    }
}
