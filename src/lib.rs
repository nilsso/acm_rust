mod divisors;
mod factorize;

// Provide submodule functions as crate functions
pub use divisors::divisors;
pub use factorize::factorize;

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
    /// Constructs a new ACM with components `a` and `b`,
    /// requiring that `a % b == a.pow(2) % b`.
    ///
    /// # Examples
    /// ```
    /// // A valid ACM
    /// assert!(std::panic::catch_unwind(|| {
    ///     acm::ArithmeticCongruenceMonoid::new(1, 4);
    /// }).is_ok());
    ///
    /// // An invalid ACM which causes a panic
    /// assert!(std::panic::catch_unwind(|| {
    ///     acm::ArithmeticCongruenceMonoid::new(2, 4);
    /// }).is_err());
    /// ```
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

    /// Returns the `a` component of the ACM.
    pub fn a(&self) -> u32 {
        self.a
    }

    /// Returns the `b` component of the ACM.
    pub fn b(&self) -> u32 {
        self.b
    }

    /// Returns `true` if `n` is an element of the ACM.
    ///
    /// # Examples
    ///
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4);
    /// assert!(acm.contains(&5));
    /// assert!(!acm.contains(&6));
    /// ```
    pub fn contains(&self, n: &u32) -> bool {
        *n == 1 || n % self.b == self.a
    }

    pub fn divisors(&self, n: u32) -> Vec<u32> {
        let mut ds = vec![];
        for d in divisors(n) {
            if self.contains(&d) {
                ds.push(d);
            }
        }
        ds
    }

    fn factorization_is_empty(&self, n: u32) -> bool {
        self.factorizations.get(&n).unwrap().is_empty()
    }

    fn add_factorization(&mut self, n: u32, factorization: Vec<u32>) {
        self.factorizations.get_mut(&n).unwrap().push(factorization)
    }

    pub fn factorize(&mut self, n: u32) -> &Vec<Vec<u32>> {
        if self.factorizations.contains_key(&n) {
            return self.factorizations.get(&n).unwrap();
        }

        self.factorizations.insert(n, vec![]);

        if self.contains(&n) {
            let n_divisors = self.divisors(n);
            for (d, q) in n_divisors
                .iter()
                .skip(1)
                .map(|d| (*d, n / d))
                .filter(|(_d, q)| n_divisors.contains(&q))
            {
                if q == 1 && self.factorization_is_empty(n) {
                    self.add_factorization(n, vec![n]);
                } else if let Some(d_fs) = self.factorize(d).first() {
                    if d_fs.len() == 1 {
                        for mut q_f in self.factorize(q).clone().into_iter() {
                            if q_f.is_empty() || &d >= q_f.last().unwrap() {
                                q_f.push(d);
                                self.add_factorization(n, q_f);
                            }
                        }
                    }
                }
            }
        }
        self.factorizations.get(&n).unwrap()
    }

    /// Generate `n` ACM elements starting at nearest element below or equal to `s`.
    ///
    /// # Examples
    ///
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4);
    /// assert_eq!(acm.elements(5, 1), vec![1, 5, 9, 13, 17]);
    /// ```
    pub fn elements(&self, n: u32, s: u32) -> Vec<u32> {
        let s = s - (s - self.a) % self.b;
        (0..n).map(|i| s + i * self.b).collect()
    }
}
