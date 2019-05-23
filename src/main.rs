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
    let yaml = load_yaml!("main.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let verbose = matches.is_present("verbose");
    if let Some(matches) = matches.subcommand_matches("F") {
        let n = parse_subcommand_arg(&matches, "n");
        if verbose {
            println!(
                "integer prime power factorization of {} : {:?}",
                n,
                factorize(n)
            );
        } else {
            println!("{:?}", factorize(n));
        }
    } else if let Some(matches) = matches.subcommand_matches("D") {
        let n = parse_subcommand_arg(&matches, "n");
        if verbose {
            println!("integer divisors of {} : {:?}", n, divisors(n));
        } else {
            println!("{:?}", divisors(n));
        }
    } else if let Some(matches) = matches.subcommand_matches("ACM") {
        let a = parse_subcommand_arg(&matches, "a");
        let b = parse_subcommand_arg(&matches, "b");
        let mut acm = ACM::new(a, b);
        if let Some(matches) = matches.subcommand_matches("F") {
            let n = parse_subcommand_arg(&matches, "n");
            if verbose {
                println!(
                    "ACM({},{}) prime power factorization of {} : {:?}",
                    a,
                    b,
                    n,
                    acm.factorize(n)
                );
            } else {
                println!("{:?}", acm.factorize(n));
            }
        } else if let Some(matches) = matches.subcommand_matches("D") {
            let n = parse_subcommand_arg(&matches, "n");
            if verbose {
                println!("ACM({},{}) divisors of {} : {:?}", a, b, n, acm.divisors(n));
            } else {
                println!("{:?}", acm.divisors(n));
            }
        } else if let Some(matches) = matches.subcommand_matches("E") {
            let n = parse_subcommand_arg(&matches, "n");
            let s = match matches.value_of("s") {
                Some(s) => s.parse().unwrap(),
                None => a,
            };
            if verbose {
                println!(
                    "ACM({},{}) {} elements starting at {} : {:?}",
                    a,
                    b,
                    n,
                    s,
                    acm.elements(n, s)
                );
            } else {
                println!("{:?}", acm.elements(n, s));
            }
        }
    }
}
