#![feature(stmt_expr_attributes, int_error_matching)]
use std::num::ParseIntError;

use clap::{load_yaml, App, ArgMatches};
use failure::Error;
use itertools::{join, Itertools};
use num_bigint::BigInt;

use acm::divisors::divisors;
use acm::factor::factor;
use acm::integers::ModClass;

type ACM = acm::ArithmeticCongruenceMonoid<BigInt>;

// Helper for loading a required clap (CLI) argument
fn req_arg(matches: &ArgMatches, arg: &'static str) -> Result<u32, ParseIntError> {
    matches.value_of(arg).unwrap().parse()
}

// Helper for loading an optional clap (CLI) argument
fn opt_arg(matches: &ArgMatches, arg: &'static str, default: u32) -> Result<u32, ParseIntError> {
    matches.value_of(arg).map_or(Ok(default), |arg| arg.parse())
}

// TODO: Fix error Result handling, types and messages.
fn cli() -> Result<(), Error> {
    let yaml = load_yaml!("acm-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let subcommand = matches.subcommand_name().unwrap();
    let matches = matches.subcommand_matches(subcommand).unwrap();
    match subcommand {
        "factor" => {
            let factors = factor(req_arg(&matches, "n")?);
            println!(
                "{}",
                join(factors.iter().map(|(p, e)| format!("({},{})", p, e)), ",")
            );
        }
        "divisors" => {
            let divisors = divisors(req_arg(&matches, "n")?);
            println!("{}", join(divisors.iter(), ","));
        }
        "acm" => {
            let a: u32 = req_arg(&matches, "a")?;
            let b: u32 = req_arg(&matches, "b")?;

            let mut acm: ACM = ACM::new(a, b).expect("");

            let subcommand = matches.subcommand_name().unwrap();
            let matches = matches.subcommand_matches(subcommand).unwrap();

            let get_n = || req_arg(&matches, "n");

            match subcommand {
                "nearest" => {
                    let n = BigInt::from(get_n()?);
                    println!("{}", acm.nearest(n));
                }
                "nth" => {
                    let n = BigInt::from(get_n()?);
                    println!("{}", acm.ith(n));
                }
                "contains" => {
                    let n = BigInt::from(get_n()?);
                    println!("{}", acm.contains(&n));
                }
                "divisors" => {
                    let n = BigInt::from(get_n()?);
                    let divisors = acm.divisors(n);
                    println!("{}", join(divisors.iter(), ","));
                }
                "factor" => {
                    let n = get_n()?;
                    let factorizations = acm.factor(n);
                    println!(
                        "[{}]",
                        join(
                            factorizations
                                .iter()
                                .map(|fs| format!("[{}]", join(fs.iter(), ","))),
                            ","
                        )
                    );
                }
                "atomic" => {
                    let n = BigInt::from(get_n()?);
                    println!("{}", acm.atomic(&n));
                }
                "n_elements" => {
                    let n = get_n()?;
                    let s = BigInt::from(opt_arg(&matches, "s", a)?);
                    println!("{}", join(acm.iter_from(s).take(n as usize), ","));
                }
                "n_atoms" => {
                    let n = get_n()?;
                    let s = BigInt::from(opt_arg(&matches, "s", a)?);
                    println!(
                        "{}",
                        join(
                            acm.iter_from(s).filter(|x| acm.atomic(x)).take(n as usize),
                            ","
                        )
                    );
                }
                "n_reducibles" => {
                    let n = get_n()?;
                    let s = BigInt::from(opt_arg(&matches, "s", a)?);
                    println!(
                        "{}",
                        join(
                            acm.iter_from(s).filter(|x| !acm.atomic(x)).take(n as usize),
                            ","
                        )
                    );
                }
                "mod_classes" => {
                    for mod_class in filter_mod_classes(&acm, matches)? {
                        println!("{:?}", mod_class);
                    }
                }
                "survey" => {
                    let max_power = req_arg(&matches, "max_power")?;
                    let max_power_sum = req_arg(&matches, "max_power_sum")?;
                    let mod_classes = filter_mod_classes(&acm, matches)?;
                    survey(&mut acm, max_power, max_power_sum, &mod_classes)?;
                }
                "survey_all" => {
                    // TODO:
                    // This is a heavy stand-in for surveying the ways in which to construct
                    // a and to construct 1 from combinations of the mod classes.
                    use std::fs::{create_dir_all, OpenOptions};
                    use std::path::Path;

                    let max_power = req_arg(&matches, "max_power")?;
                    let max_power_sum = req_arg(&matches, "max_power_sum")?;
                    let out_path = Path::new(matches.value_of("out_dir").unwrap());
                    create_dir_all(out_path)?;
                    let mod_classes = acm.mod_classes().clone();
                    for i in 1..mod_classes.len() {
                        for mcs in mod_classes.iter().copied().combinations(i) {
                            let file_name =
                                format!("{}.csv", join(mcs.iter().map(|mc| mc.a()), "-"));
                            let file_path = out_path.join(Path::new(&file_name));
                            let mut file = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open(file_path)?;
                            survey_to(&mut file, &mut acm, max_power, max_power_sum, &mcs)?;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn main() {
    if let Err(err) = cli() {
        println!("Error: {:?}", err);
    }
}

// Helper for filtering the mod classes of an ACM via Clap argument matches
fn filter_mod_classes(acm: &ACM, matches: &clap::ArgMatches) -> Result<Vec<ModClass>, Error> {
    if let Some(mod_class_strings) = matches.values_of("mod_classes") {
        // If mod_classes specified,
        // filter the ACM mod classes over those specified
        let mod_classes: Result<Vec<u32>, ParseIntError> = mod_class_strings
            .into_iter()
            .map(str::parse::<u32>)
            .collect();
        let mod_classes: Vec<u32> = mod_classes?;
        Ok(acm
            .mod_classes()
            .iter()
            .filter(|mc| mod_classes.contains(&mc.a()))
            .copied()
            .collect())
    } else {
        // Otherwise just use the ACM mod classes
        Ok(acm.mod_classes().clone())
    }
}

// Helper for surveying ACM mod class combinations
fn survey_to<T: std::io::Write>(
    os: &mut T,
    acm: &mut ACM,
    max_power: u32,
    max_power_sum: u32,
    _mod_classes: &Vec<ModClass>,
) -> Result<(), Error> {
    // TODO: This is stuck in an infinite somewhere, for ACMs like M_{13,26}
    // let ps = (2..acm.b()).map(|x| BigInt::from(x)).collect();
    let mut ps = vec![];
    let mut i = BigInt::from(2);
    while &i < acm.b() {
        ps.push(i.clone());
        i += 1;
    }
    let mod_classes = ps.clone();
    // let ps = mod_classes
    //     .iter()
    //     .map(|mc| BigInt::from(mc.first_prime().unwrap()))
    //     .collect::<Vec<BigInt>>();
    writeln!(
        os,
        "n,atomic,{}",
        // join(mod_classes.iter().map(|mc| mc.a()), ",")
        join(mod_classes.iter(), ",")
    )?;
    for es in (0..ps.len())
        .map(|_| 0..max_power + 1)
        .multi_cartesian_product()
        .filter(|es| es.iter().sum::<u32>() <= max_power_sum)
    {
        let n: BigInt = ps.iter().zip(es.iter()).map(|(p, e)| p.pow(*e)).product();
        if acm.contains(&n) {
            let atomic = acm.atomic(&n);
            let e_string = join(es.iter(), ",");
            writeln!(os, "{},{},{}", n, atomic, e_string)?;
        }
    }
    Ok(())
}

fn survey(
    acm: &mut ACM,
    max_power: u32,
    max_power_sum: u32,
    mod_classes: &Vec<ModClass>,
) -> Result<(), Error> {
    survey_to(
        &mut std::io::stdout(),
        acm,
        max_power,
        max_power_sum,
        mod_classes,
    )?;
    Ok(())
}
