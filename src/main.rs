#![feature(bool_to_option, step_trait)]
#![allow(unused_imports)]
use acm::{integers::GCD, ArithmeticCongruenceMonoid as ACM};

//use itertools::iproduct;
use itertools::Itertools;

use std::iter::repeat;

fn mod_classes(a: i32, b: i32) -> Vec<i32> {
    (2..b)
        .filter(|&i| i.gcd(b) == 1 || i != a && a % i == 0)
        .collect()
}

fn main() {
    let a = 6;
    let b = 10;
    let m = 4;

    let mut acm = ACM::new(a, b).unwrap();
    let ps = mod_classes(a, b);

    println!("n,atomic,{}", ps.iter().map(|p| p.to_string()).join(","));
    for es in (0..ps.len()).map(|_| 0..m).multi_cartesian_product() {
        let n: i32 = ps.iter().zip(es.iter()).map(|(p, e)| p.pow(*e)).product();
        let atomic = acm.atomic(&n);
        let e_string = es.into_iter().map(|e| e.to_string()).join(",");
        if acm.contains(&n) {
            println!("{},{},{}", n, atomic, e_string);
        }
    }
}
