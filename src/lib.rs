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

impl ArithmeticCongruenceMonoid {
    pub fn new(a: u32, b: u32) -> ArithmeticCongruenceMonoid {
        if a % b != a.pow(2) % b {
            // TODO: Proper error handling. Use `Result` instead causing of panic.
            panic!("a and a^2 must be equivalent mod b");
        }
        ArithmeticCongruenceMonoid {
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
        n % self.b == self.a
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

    //fn factorize_helper(&self,
                        //d: u32,
                        //divisors: &Vec<u32>,
                        //factorizations: &Vec<Vec<u32>>) -> &Vec<Vec<u32>> {
        //if divisors.contains(n) {
        //}
        //factorizations
    //}

    pub fn factorize(&mut self, n: u32) -> &Vec<Vec<u32>> {
        //dbg!(&n);

        // Returned if cached
        if self.factorizations.contains_key(&n) {
            let n_factorizations = self.factorizations.get(&n).unwrap();
            dbg!((&n, &n_factorizations));
            return n_factorizations;
        }

        // Instantiate new factorization vector
        //let mut n_factorizations = vec![];
        self.factorizations.insert(n, vec![]);
        let mut n_factorizations = self.factorizations.get_mut(&n).unwrap();

        // Skip if not an ACM element
        if self.contains(n) {
            let n_divisors = self.divisors(n);
            let dq_it = n_divisors
                .iter()
                .skip(1)
                .map(|d| (*d, n / d))
                .filter(|(_d, q)| n_divisors.contains(&q));
            for (d, q) in dq_it {
                //dbg!((&n, &d, &q, &n_factorizations));
                if q == 1 && n_factorizations.is_empty() {
                    n_factorizations.push(vec![n]);
                } else if self.factorize(d).first().unwrap().len() == 1 {
                    for mut q_factorization in self.factorize(q).iter().cloned() {
                        dbg!((&d, &q_factorization));
                        if q_factorization.is_empty() || &d >= q_factorization.last().unwrap() {
                            q_factorization.push(d);
                            n_factorizations.push(q_factorization);
                            dbg!((&n, &n_factorizations));
                        }
                    }
                }
            }
        }

        // Move into hash map and return reference from hash map
        dbg!((&n, &n_factorizations));
        //self.factorizations.insert(n, n_factorizations);
        //self.factorizations.get(&n).unwrap()
        n_factorizations
    }
}
