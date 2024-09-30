use paf::Reader;
use std::io::Write;
use std::path::PathBuf;

const HELP: &str = "\
psd - Calculate the per-sequence divergence from a PAF file
Max Brown <max.carter-brown@aru.ac.uk>

USAGE:
  psd [-h] [PAF]

FLAGS:
  -h, --help            Prints help information
  -i, --individual      Print individual sequence divergence 
                        values. These will be sorted by the 
                        query input order.
ARGS:
  <PAF>                 Path to PAF file
";

struct PSDArgs {
    individual: bool,
    paf: PathBuf,
}

fn parse_args() -> Result<PSDArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = PSDArgs {
        individual: pargs.contains(["-i", "--individual"]),
        paf: pargs.free_from_str()?,
    };

    if args.paf == PathBuf::default() {
        return Err(pico_args::Error::MissingArgument);
    }

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

struct PAFSorter(Vec<paf::PafRecord>);

impl PAFSorter {
    fn sort_by_query_start(&mut self) {
        self.0.sort_by(|a, b| a.query_start().cmp(&b.query_start()));
    }

    fn sort_by_query_name(&mut self) {
        self.0.sort_by(|a, b| a.query_name().cmp(&b.query_name()));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    if args.individual {
        paf_individual(args)?;
    } else {
        paf(args)?;
    }

    Ok(())
}

fn paf_individual(args: PSDArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(args.paf)?;
    let mut records: Vec<paf::PafRecord> = Vec::new();

    let mut stdout = std::io::stdout();

    for record in reader.records() {
        let record = record?;
        records.push(record);
    }

    let mut sorted = PAFSorter(records);
    // sort on both name and query
    sorted.sort_by_query_start();
    sorted.sort_by_query_name();

    for (record1, record2) in sorted.0.iter().zip(sorted.0.iter().skip(1)) {
        // if there are overlaps between the two records, skip
        if record1.query_end() > record2.query_start() {
            continue;
        }

        let de = match record1.de() {
            Some(d) => *d,
            None => {
                // error out here
                return Err(
                    "Could not get DE (the gap-compressed per sequence divergence) from record"
                        .into(),
                );
            }
        };

        let qname = record1.query_name();

        let _ = writeln!(
            stdout,
            "{}\t{}\t{}\t{}",
            qname,
            record1.query_start(),
            record1.query_end(),
            de
        );
    }

    Ok(())
}

fn paf(args: PSDArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(args.paf)?;
    let mut stdout = std::io::stdout();

    let mut sum_aln_len_de = 0.0;
    let mut sum_aln_len = 0.0;

    for record in reader.records() {
        let record = record?;

        let de = match record.de() {
            Some(d) => *d,
            None => {
                // error out here
                return Err(
                    "Could not get DE (the gap-compressed per sequence divergence) from record"
                        .into(),
                );
            }
        };

        sum_aln_len_de += record.alignment_block_len() as f64 * de;
        sum_aln_len += record.alignment_block_len() as f64;
    }

    let psd = sum_aln_len_de / sum_aln_len;
    let _ = writeln!(stdout, "{}", psd);

    Ok(())
}
