extern crate clap;

use acm_rust::divisors::divisors;
use acm_rust::prime_factors::prime_factors;
use acm_rust::ArithmeticCongruenceMonoid as ACM;
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("acm_rust")
        .about("ACM test program")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("ACM")
                .about("ACM subcommand")
                .version("0.1")
                .author("Nils Olsson <nilso@enosis.net>")
                .arg(Arg::with_name("a").help("Component a").required(true))
                .arg(Arg::with_name("b").help("Component b").required(true))
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("F")
                        .arg(Arg::with_name("n").help("Integer").required(true)),
                )
                .subcommand(
                    SubCommand::with_name("D")
                        .arg(Arg::with_name("n").help("Integer").required(true)),
                ),
        )
        .subcommand(
            SubCommand::with_name("F")
                .about("Prime factorization")
                .arg(Arg::with_name("n").help("Integer").required(true)),
        )
        .subcommand(
            SubCommand::with_name("D")
                .about("Divisors")
                .arg(Arg::with_name("n").help("Integer").required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("F") {
        let n = parse_subcommand_arg(&matches, "n");
        println!("prime factors of {} : {:?}", n, prime_factors(n));
    } else if let Some(matches) = matches.subcommand_matches("D") {
        let n = parse_subcommand_arg(&matches, "n");
        println!("divisors of {} : {:?}", n, divisors(n));
    } else if let Some(matches) = matches.subcommand_matches("ACM") {
        let a = parse_subcommand_arg(&matches, "a");
        let b = parse_subcommand_arg(&matches, "b");
        let mut acm = ACM::new(a, b);
        if let Some(matches) = matches.subcommand_matches("F") {
            let n = parse_subcommand_arg(&matches, "n");
            println!("{:?}", acm.factorize(n));
        } else if let Some(matches) = matches.subcommand_matches("D") {
            let n = parse_subcommand_arg(&matches, "n");
            println!("{:?}", acm.divisors(n));
            //println!("{:?} divisors of {} : {:?}", &acm, n, acm.divisors(n));
        }
    }
}

fn parse_subcommand_arg(m: &clap::ArgMatches<'_>, arg: &str) -> u32 {
    m.value_of(arg).unwrap().parse().unwrap()
}
