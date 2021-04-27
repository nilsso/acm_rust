use acm::{factor::factor, ArithmeticCongruenceMonoid as ACM};

fn main() {
    let mut acm = ACM::new(6, 10).unwrap();

    let f = |a: u32, b, c, d, e|-> i32 {
        (2_i32.pow(a) * 3_i32.pow(b) * 7_i32.pow(c)) * (3_i32 * 7_i32).pow(d) * (3_i32.pow(4)).pow(e)
    };

    let n = f(4, 0, 0, 0, 1);

    acm.factor(n);

    // println!("{}", n);
    // println!("{:?}", factor(n));
    println!("{:?}", acm.factor(n));
    // println!("{:#?}", acm.factorizations);
}