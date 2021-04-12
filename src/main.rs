#![allow(unused_imports, dead_code)]
use std::str::FromStr;

use num_bigint::BigInt;

use acm::ArithmeticCongruenceMonoid as ACM;

fn main() {
    let mut args = std::env::args().skip(1);

    //type T = BigInt;
    type T = i32;

    let a = T::from_str(&args.next().unwrap()).unwrap();
    let b = T::from_str(&args.next().unwrap()).unwrap();
    let n = T::from_str(&args.next().unwrap()).unwrap();

    let mut acm = ACM::new(a, b).unwrap();

    println!("{}", n);
    println!("{:?}", acm.factor(n));
}
