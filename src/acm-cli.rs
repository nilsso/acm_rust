use acm::{divisors, factorize, ArithmeticCongruenceMonoid as ACM};
use clap::{load_yaml, App, ArgMatches};
use failure::Error;
use plotlib::{page::Page, scatter::Scatter, view::ContinuousView};

fn cli() -> Result<(), Error> {
    let req_arg = |matches: &ArgMatches, arg| matches.value_of(arg).unwrap().parse();
    let opt_arg = |matches: &ArgMatches, arg, default| {
        matches.value_of(arg).map_or(Ok(default), |arg| arg.parse())
    };
    let get_n = |matches: &ArgMatches| req_arg(&matches, "n");

    let yaml = load_yaml!("acm-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let subcommand = matches.subcommand_name().unwrap();
    let matches = matches.subcommand_matches(subcommand).unwrap();
    match subcommand {
        "factorize" => println!("{:?}", factorize(get_n(&matches)?)),
        "divisors" => println!("{:?}", divisors(get_n(&matches)?)),
        "acm" => {
            let a = req_arg(&matches, "a")?;
            let b = req_arg(&matches, "b")?;
            let get_s = |matches: &ArgMatches| opt_arg(&matches, "s", a);
            let plot = |data: &[(f64, f64)], x_min, x_max, y_min, y_max| {
                let (w, h) = term_size::dimensions().unwrap();
                let (w, h) = ((w - 10) as u32, ((h - 10) / 2) as u32);
                //println!("{} {}", w, h);
                let s = Scatter::from_slice(data);
                let v = ContinuousView::new().add(&s).x_range(x_min, x_max).y_range(y_min, y_max);
                println!("{}", Page::single(&v)
                         .dimensions(w, h)
                         .to_text()
                         .unwrap());
            };

            let mut acm: ACM = ACM::new(a, b)?;

            let subcommand = matches.subcommand_name().unwrap();
            let matches = matches.subcommand_matches(subcommand).unwrap();
            let n = get_n(&matches)?;
            match subcommand {
                "nearest" => println!("{}", acm.nearest(n)),
                "nth" => println!("{}", acm.nth(n)),
                "contains" => println!("{}", acm.contains(n)),
                "factorize" => println!("{:?}", acm.factorize(n)),
                "divisors" => println!("{:?}", acm.divisors(n)),
                "atomic" => println!("{}", acm.atomic(n)),
                "n_elements" => println!("{:?}", acm.n_elements(n, get_s(&matches)?)),
                "n_atoms" => {
                    let s = get_s(&matches)?;
                    let elements = acm.n_elements(n, s);
                    let x_max = *elements.last().unwrap();
                    let data: Vec<(f64, f64)> = elements
                        .iter()
                        .map(|&n| (n as f64, if acm.atomic(n) { 1.0 } else { 0.0 }))
                        .collect();
                    plot(data.as_slice(), s as f64, x_max as f64, -1.0, 1.0);
                }
                "atomic_density" => {
                    let s = get_s(&matches)?;
                    let atoms = acm.n_atoms(n, s);
                    let metric = |(&a1, &a2)| a2 - a1;
                    let density = atoms
                        .iter()
                        .zip(atoms.iter().skip(1))
                        .map(metric)
                        .collect::<Vec<u64>>();
                    let y_max = *density.iter().max().unwrap() as f64;
                    let data: Vec<(f64, f64)> = density
                        .into_iter()
                        .enumerate()
                        .map(|(i, n)| ((i as u64 + s) as f64, n as f64))
                        .collect();
                    plot(data.as_slice(), s as f64, data.len() as f64, 0.0, y_max);
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
