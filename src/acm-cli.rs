#![feature(stmt_expr_attributes)]
use clap::{load_yaml, App, ArgMatches};
use failure::Error;

use std::num::ParseIntError;
use std::{fmt, fmt::Display};

use acm::divisors::divisors;
use acm::factor::factor;
use acm::ArithmeticCongruenceMonoid as ACM;

fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    }
    for i in 2..=(n as f32).sqrt().round() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn first_prime(a: u64, b: u64) -> u64 {
    let mut n = a;
    loop {
        if is_prime(n) {
            return n;
        }
        n += b;
    }
}

#[derive(Debug)]
struct ModClass {
    base: u64,
    modulo: u64,
    first_prime: u64,
}

impl ModClass {
    pub fn new(base: u64, modulo: u64) -> Self {
        Self {
            base,
            modulo,
            first_prime: first_prime(base, modulo),
        }
    }
}

impl Display for ModClass {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}]", self.base)
        //if self.base == self.first_prime {
        //write!(fmt, "[{}]", self.base)
        //} else {
        //write!(fmt, "[{}≡{}]", self.base, self.first_prime)
        //}
    }
}

fn req_arg(matches: &ArgMatches, arg: &'static str) -> Result<u64, ParseIntError> {
    matches.value_of(arg).unwrap().parse()
}

fn opt_arg(matches: &ArgMatches, arg: &'static str, default: u64) -> Result<u64, ParseIntError> {
    matches.value_of(arg).map_or(Ok(default), |arg| arg.parse())
}

fn cli() -> Result<(), Error> {
    let yaml = load_yaml!("acm-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let subcommand = matches.subcommand_name().unwrap();
    let matches = matches.subcommand_matches(subcommand).unwrap();
    match subcommand {
        "factor" => println!("{:?}", factor(req_arg(&matches, "n")?)),
        "divisors" => println!("{:?}", divisors(req_arg(&matches, "n")?)),
        "acm" => {
            let a = req_arg(&matches, "a")?;
            let b = req_arg(&matches, "b")?;
            let mut acm: ACM = ACM::new(a, b)?;

            let subcommand = matches.subcommand_name().unwrap();
            let matches = matches.subcommand_matches(subcommand).unwrap();
            let n = req_arg(&matches, "n")?;
            match subcommand {
                "nearest" => println!("{}", acm.nearest(n)),
                "nth" => println!("{}", acm.nth(n)),
                "contains" => println!("{}", acm.contains(n)),
                "factor" => println!("{:?}", acm.factor(n)),
                "divisors" => println!("{:?}", acm.divisors(n)),
                "atomic" => println!("{}", acm.atomic(n)),
                "n_elements" => println!("{:?}", acm.n_elements(n, opt_arg(&matches, "s", a)?)),
                "n_atoms" => println!("{:?}", acm.n_atoms(n, opt_arg(&matches, "s", a)?)),
                "n_reducibles" => println!("{:?}", acm.n_reducibles(n, opt_arg(&matches, "s", a)?)),
                "mod_classes" => {
                    //let mut mod_classes: Vec<ModClass> = acm
                    let mod_classes: Vec<ModClass> = acm
                        .prime_factor_mod_bases()
                        .into_iter()
                        .map(|base| ModClass::new(base, acm.b()))
                        .collect();

                    //mod_classes[0].first_prime = 61;

                    let mut combinations: Vec<Vec<&ModClass>> = Vec::new();

                    fn helper<'a>(
                        mod_classes: &'a Vec<ModClass>,
                        combinations: &mut Vec<Vec<&'a ModClass>>,
                        curr: Vec<&'a ModClass>,
                        length: usize,
                        max_length: usize,
                        i: usize,
                    ) {
                        combinations.push(curr.clone());
                        if length < max_length {
                            for (i, mc) in mod_classes
                                .iter()
                                .filter(|mc| mc.base > 1)
                                .enumerate()
                                .skip(i)
                            {
                                let mut next = curr.clone();
                                next.push(mc);
                                helper(mod_classes, combinations, next, length + 1, max_length, i);
                            }
                        }
                    };

                    for (i, mc) in mod_classes.iter().enumerate() {
                        helper(&mod_classes, &mut combinations, vec![mc], 1, n as usize, i);
                    }

                    use std::collections::BTreeMap;

                    let counter: BTreeMap<u64, usize> =
                        mod_classes.iter().map(|mc| (mc.base, 0)).collect();

                    println!(
                        "n,atomic,{}",
                        mod_classes
                            .iter()
                            .map(|a| format!("{}", a.base))
                            .collect::<Vec<String>>()
                            .join(",")
                    );

                    for combination in combinations {
                        let n: u64 = combination.iter().map(|mc| mc.base).product();
                        let mut counter = counter.clone();
                        for mc in combination {
                            *counter.get_mut(&mc.base).unwrap() += 1;
                        }
                        let s = counter
                            .into_iter()
                            .map(|(_, b)| format!("{}", b))
                            .collect::<Vec<String>>()
                            .join(",");
                        if acm.contains(n) {
                            println!("{},{},{:8}", n, acm.atomic(n), s);
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
