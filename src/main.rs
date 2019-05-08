#[macro_use]
extern crate clap;

use acm_rust::prime_factors::prime_factors;
use acm_rust::divisors::divisors;
use acm_rust::ArithmeticCongruenceMonoid as ACM;

fn main() {
    let matches = clap_app!(acm_rust =>
        (version: "0.1")
        (author: "Nils Olsson <nilso@enosis.net>")
        (about: "ACM test program")
        (@arg a: +required "Component a")
        (@arg b: +required "Component b")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand F =>
            (about: "Prime factorization")
            (@arg n: +required "Number")
        )
        (@subcommand D =>
            (about: "Divisors")
            (@arg n: +required "Number")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("F") {
        let n = n(matches);
        println!("prime factors of {} : {:?}", n, divisors(n));
    }
    else if let Some(matches) = matches.subcommand_matches("D") {
        let n = n(matches);
        println!("divisors of {} : {:?}", n, divisors(n));
    }
    else {
        let a = matches.value_of("a").unwrap().parse().unwrap();
        let b = matches.value_of("b").unwrap().parse().unwrap();
        let acm = ACM::new(a, b);
        println!("acm({},{})", acm.a(), acm.b());
    }
}

fn n(m: &clap::ArgMatches<'_>) -> u32 {
    m.value_of("n").unwrap().parse().unwrap()
}
