#[macro_use]
extern crate clap;

use acm_rust::factorize;
use acm_rust::divisors;
use acm_rust::ArithmeticCongruenceMonoid as ACM;
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
                factorize::factorize(n)
            );
        } else {
            println!("{:?}", factorize::factorize(n));
        }
    } else if let Some(matches) = matches.subcommand_matches("D") {
        let n = parse_subcommand_arg(&matches, "n");
        if verbose {
            println!("integer divisors of {} : {:?}", n, divisors::divisors(n));
        } else {
            println!("{:?}", divisors::divisors(n));
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
        }
    }
}
