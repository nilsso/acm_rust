pub mod divisors;
pub mod prime_factors;

use common_macros::hash_map;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ArithmeticCongruenceMonoid {
    a: u32,
    b: u32,
    factorizations: HashMap<u32, Vec<Vec<u32>>>,
}

type ACM = ArithmeticCongruenceMonoid;

impl ArithmeticCongruenceMonoid {
    pub fn new(a: u32, b: u32) -> ACM {
        if a % b != a.pow(2) % b {
            // TODO: Proper error handling. Use `Result` instead causing of panic.
            panic!("a and a^2 must be equivalent mod b");
        }
        ACM {
            a: a % b,
            b,
            factorizations: hash_map! {
                1 => vec![vec![]]
            },
        }
    }

    pub fn a(&self) -> u32 {
        self.a
    }

    pub fn b(&self) -> u32 {
        self.b
    }

    pub fn contains(&self, n: u32) -> bool {
        n == 1 || n % self.b == self.a
    }

    pub fn divisors(&self, n: u32) -> Vec<u32> {
        let mut ds = vec![];
        for d in divisors::divisors(n) {
            if self.contains(d) {
                ds.push(d);
            }
        }
        ds
    }

    pub fn factorization_is_empty(&self, n: u32) -> bool {
        self.factorizations.get(&n).unwrap().is_empty()
    }

    fn add_factorization(&mut self, n: u32, factorization: Vec<u32>) {
        self.factorizations.get_mut(&n).unwrap().push(factorization)
    }

    pub fn factorize(&mut self, n: u32) -> &Vec<Vec<u32>> {
        // Returned if cached
        if self.factorizations.contains_key(&n) {
            return self.factorizations.get(&n).unwrap();
        }

        // Instantiate new factorization vector
        self.factorizations.insert(n, vec![]);

        if self.contains(n) {
            let n_divisors = self.divisors(n);

            for (d, q) in n_divisors.iter().skip(1).map(|d| (*d, n / d)) {
                if !n_divisors.contains(&q) {
                    continue;
                }
                if q == 1 && self.factorization_is_empty(n) {
                    self.add_factorization(n, vec![n]);
                } else if let Some(d_factorizations) = self.factorize(d).first() {
                    if d_factorizations.len() == 1 {
                        for mut q_factorization in self.factorize(q).clone().into_iter() {
                            if q_factorization.is_empty() || &d >= q_factorization.last().unwrap() {
                                q_factorization.push(d);
                                self.add_factorization(n, q_factorization);
                            }
                        }
                    }
                }
            }
        }
        self.factorizations.get(&n).unwrap()
    }
}
