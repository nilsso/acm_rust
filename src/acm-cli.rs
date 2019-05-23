//! Test

use acm::divisors;
use acm::factorize;
use acm::ArithmeticCongruenceMonoid as ACM;

#[macro_use]
extern crate clap;
use clap::App;

fn parse_subcommand_arg(m: &clap::ArgMatches<'_>, arg: &str) -> u32 {
    m.value_of(arg).unwrap().parse().unwrap()
}

fn main() {
    let yaml = load_yaml!("acm-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("factorize") {
        let n = parse_subcommand_arg(&matches, "n");
        println!("{:?}", factorize(n));
    } else if let Some(matches) = matches.subcommand_matches("divisors") {
        let n = parse_subcommand_arg(&matches, "n");
        println!("{:?}", divisors(n));
    } else if let Some(matches) = matches.subcommand_matches("acm") {
        let a = parse_subcommand_arg(&matches, "a");
        let b = parse_subcommand_arg(&matches, "b");
        match ACM::new(a, b) {
            Ok(mut acm) => {
                if let Some(matches) = matches.subcommand_matches("elements") {
                    let n = parse_subcommand_arg(&matches, "n");
                    let s = match matches.value_of("s") {
                        Some(s) => s.parse().unwrap(),
                        None => a,
                    };
                    println!("{:?}", acm.elements(n, s));
                } else if let Some(matches) = matches.subcommand_matches("contains") {
                    let n = parse_subcommand_arg(&matches, "n");
                    println!("{}", acm.contains(n));
                } else if let Some(matches) = matches.subcommand_matches("factorize") {
                    let n = parse_subcommand_arg(&matches, "n");
                    println!("{:?}", acm.factorize(n));
                } else if let Some(matches) = matches.subcommand_matches("divisors") {
                    let n = parse_subcommand_arg(&matches, "n");
                    println!("{:?}", acm.divisors(n));
                } else if let Some(matches) = matches.subcommand_matches("atomic") {
                    let n = parse_subcommand_arg(&matches, "n");
                    println!("{}", acm.atomic(n));
                } else if let Some(matches) = matches.subcommand_matches("atoms") {
                    let n = parse_subcommand_arg(&matches, "n");
                    let s = match matches.value_of("s") {
                        Some(s) => s.parse().unwrap(),
                        None => a,
                    };
                    println!("{:?}", acm.atoms(n, s));
                }
            }
            Err(err) => println!("Invalid ACM: {}", err),
        }
    }
}
