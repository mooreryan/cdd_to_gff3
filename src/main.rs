use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

/// Parse CDD search output to gff3 format
///
/// Longer thing arositenarsoitn  arositnaroite roist arist a.
#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opts {
    /// CDD search input
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    // Sometimes the header will have a Definition column as well
    let expected_header_start = "Query\tHit type\tPSSM-ID\tFrom\tTo\tE-Value\tBitscore\tAccession\tShort name\tIncomplete\tSuperfamily";
    let num_good_cols = 11;

    let opts = Opts::from_args();
    eprintln!("{:?}", opts);

    let file = match File::open(&opts.input) {
        Err(why) => panic!("Couldn't open file {}: {}", &opts.input.display(), why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut gff_id = 0;
    // Sometimes, cdd search puts [] at the end of the query, so for query name we match anything other than [] at the end
    let query_regex = Regex::new(r"^Q#\d+ - >(?P<query>[^\[\]]+)\[*\]*").unwrap();

    println!("##gff-version 3");

    for (idx, line) in reader.lines().enumerate() {
        let line = match line {
            Err(why) => panic!("Couldn't read line {}: {}", idx, why),
            Ok(line) => line,
        };

        if !line.starts_with('#') && !line.is_empty() {
            if line.starts_with("Query") && !line.starts_with(expected_header_start) {
                panic!(
                    "Header line should start with '{}', found {}",
                    expected_header_start, line
                );
            } else if !line.starts_with("Query") {
                let ary: Vec<&str> = line.split('\t').take(num_good_cols).collect();

                if ary.len() != num_good_cols {
                    panic!(
                        "line {} had only {} cols, expected: {}",
                        idx + 1,
                        ary.len(),
                        num_good_cols
                    );
                }

                let query = query_regex.replace(ary[0], "$query").to_string();
                let hit_type = ary[1];
                let from = ary[3];
                let to = ary[4];
                let evalue = ary[5];
                let short_name = ary[8];

                if hit_type == "superfamily" {
                    let desc = format!("ID={};Name={};HitType={}", gff_id, short_name, hit_type);

                    gff_id += 1;

                    let v: Vec<&str> = vec![
                        &query,
                        "cdd_search",
                        "region",
                        from,
                        to,
                        evalue,
                        ".",
                        ".",
                        &desc,
                    ];
                    let new_line = v.join("\t");

                    println!("{}", new_line);
                }
            }
        }
    }
}
