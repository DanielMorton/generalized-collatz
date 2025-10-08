use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "collatz", about = "Extended Collatz conjecture calculator")]
pub struct Args {
    #[structopt(short, long)]
    pub n: u64,

    #[structopt(short = "s", long = "start")]
    pub a_start: u64,

    #[structopt(short = "e", long = "end")]
    pub a_end: u64,

    #[structopt(long, required_unless = "write-cycle")]
    pub write_table: bool,

    #[structopt(long, required_unless = "write-table")]
    pub write_cycle: bool,
}

impl Args {
    pub fn parse() -> Result<Self> {
        Ok(Self::from_args())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse() {
        let args = Args::from_iter(&["test", "-n", "100", "-s", "3", "-e", "5", "--write-table"]);
        assert_eq!(args.n, 100);
        assert_eq!(args.a_start, 3);
        assert_eq!(args.a_end, 5);
        assert!(args.write_table);
        assert!(!args.write_cycle);
    }
}
