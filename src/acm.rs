mod divisors;
mod factorize;

// Provide submodule functions as crate functions
pub use divisors::divisors;
pub use factorize::factorize;

use common_macros::hash_map;
use failure::Fail;
use std::collections::HashMap;

/// Error to encapsulate invalid ACM construction parameters.
#[derive(Fail, Debug)]
#[fail(display = "{} incongruent to {} modulus {}.", _0, _1, _2)]
pub struct ACMError(u64, u64, u64);

/// Arithmetic congruence monoid implementation.
#[derive(Debug)]
pub struct ArithmeticCongruenceMonoid {
    a: u64,
    b: u64,
    factorizations: HashMap<u64, Vec<Vec<u64>>>,
}

type ACM = ArithmeticCongruenceMonoid;

impl ArithmeticCongruenceMonoid {
    /// Construct a new ACM with components $a$ and $b$ satisfying $a\equiv a^2\pmod b$.
    ///
    /// # Examples
    /// ```
    /// // A valid ACM (1 % 4 == 1 == 1*1 % 4)
    /// assert!(acm::ArithmeticCongruenceMonoid::new(1, 4).is_ok());
    ///
    /// // An invalid ACM (2 % 4 == 2 != 0 == 2*2 % 4)
    /// assert!(acm::ArithmeticCongruenceMonoid::new(2, 4).is_err());
    /// ```
    pub fn new(a: u64, b: u64) -> Result<ACM, ACMError> {
        if a % b == a.pow(2) % b {
            Ok(ACM {
                a: a % b,
                b,
                factorizations: hash_map! {
                    1 => vec![vec![]]
                },
            })
        } else {
            Err(ACMError(a, a * a, b))
        }
    }

    /// Returns the $a$ component of the ACM.
    pub fn a(&self) -> u64 {
        self.a
    }

    /// Returns the $b$ component of the ACM.
    pub fn b(&self) -> u64 {
        self.b
    }

    /// Returns the nearst ACM element less-than or equal to $s$.
    /// If $s < a$, returns $a$.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.nearest(0), 1);
    /// assert_eq!(acm.nearest(1), 1);
    /// assert_eq!(acm.nearest(5), 5);
    /// assert_eq!(acm.nearest(6), 5);
    /// ```
    pub fn nearest(&self, s: u64) -> u64 {
        if s >= self.a {
            s - (s - self.a) % self.b
        } else {
            self.a
        }
    }

    /// Returns the $n$th ACM element.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.nth(1), 1);
    /// assert_eq!(acm.nth(2), 5);
    /// assert_eq!(acm.nth(57), 225);
    /// ```
    pub fn nth(&self, n: u64) -> u64 {
        self.a + self.b * (n as u64 - 1)
    }

    /// Get ACM element index of an integer.
    pub fn index(&self, n: u64) -> Option<u64> {
        if self.contains(n) {
            Some(((n - self.a) / self.b + 1) as u64)
        } else {
            None
        }
    }

    /// Generate `n` ACM elements starting at nearest element less-than or equal to `s`.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.n_elements(5, 1), [1, 5, 9, 13, 17]);
    /// ```
    pub fn n_elements(&self, n: u64, s: u64) -> Vec<u64> {
        let s = self.nearest(s);
        (0..n).map(|i| s + i * self.b).collect()
    }

    /// Returns `true` if `n` is an element of the ACM.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert!( acm.contains(5));
    /// assert!(!acm.contains(6));
    /// ```
    pub fn contains(&self, n: u64) -> bool {
        n == 1 || n % self.b == self.a
    }

    /// Returns the ACM element divisors of an integer `n`.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.divisors(225), [1, 9, 5, 25, 45, 225]);
    /// ```
    pub fn divisors(&self, n: u64) -> Vec<u64> {
        let mut ds = vec![];
        for d in divisors(n) {
            if self.contains(d) {
                ds.push(d);
            }
        }
        ds
    }

    /// Returns a reference to the vector of ACM atom factorizations of an integer `n`.
    /// If `n` is not an element of the ACM then the vector will be empty.
    /// Because factorization results are stored internally to the ACM in order to reduce
    /// computational costs, using [`factorize`] requires that the ACM binding be declared mutable.
    ///
    /// # Examples
    /// ```
    /// let mut acm = acm::ArithmeticCongruenceMonoid::new(3, 6).unwrap();
    /// assert_eq!(acm.factorize(1),   &[[]]);
    /// assert_eq!(acm.factorize(2),   &[[]; 0]);
    /// assert_eq!(acm.factorize(3),   &[[3]]);
    /// assert_eq!(acm.factorize(9),   &[[3, 3]]);
    /// assert_eq!(acm.factorize(225), &[[15, 15], [3, 75]]);
    /// ```
    /// [`factorize`]: ./struct.ArithmeticCongruenceMonoid.html#methods.factorize
    pub fn factorize(&mut self, n: u64) -> &Vec<Vec<u64>> {
        // TODO: Further optimize
        if self.factorizations.contains_key(&n) {
            return self.factorizations.get(&n).unwrap();
        }

        self.factorizations.insert(n, vec![]);

        if self.contains(n) {
            let n_ds = self.divisors(n);
            for (d, q) in n_ds[1..n_ds.len() - 1]
                .iter()
                .map(|d| (*d, n / d))
                // Considering squaring both sides (problem is with overflow)
                .filter(|(d, q)| *d >= ((*q as f32).sqrt() as u64))
            {
                if let Some(d_fs) = self.factorize(d).first() {
                    if d_fs.len() == 1 {
                        for mut q_f in self.factorize(q).clone().into_iter() {
                            if q_f.is_empty() || &d >= q_f.last().unwrap() {
                                q_f.push(d);
                                self.factorizations.get_mut(&n).unwrap().push(q_f);
                            }
                        }
                    }
                }
            }
            if self.factorizations.get(&n).unwrap().is_empty() {
                self.factorizations.get_mut(&n).unwrap().push(vec![n]);
            }
        }
        self.factorizations.get(&n).unwrap()
    }

    /// Returns `true` if `n` is atomic under the ACM (is an ACM element, and cannot be expressed
    /// as a product of smaller ACM atoms).
    /// Because of underlying usage of [`factorize`], using [`atomic`] requires that the ACM binding be
    /// declared mutable.
    ///
    /// # Examples
    /// ```
    /// let mut acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert!( acm.contains(5)  &&  acm.atomic(5));
    /// assert!(!acm.contains(15) && !acm.atomic(15));
    /// assert!( acm.contains(25) && !acm.atomic(25));
    /// ```
    /// [`factorize`]: ./struct.ArithmeticCongruenceMonoid.html#method.factorize
    /// [`atomic`]: ./struct.ArithmeticCongruenceMonoid.html#method.atomic
    pub fn atomic(&mut self, n: u64) -> bool {
        if !self.contains(n) {
            return false;
        }
        let n_fs = self.factorize(n);
        n_fs.len() == 1 && n_fs.first().unwrap().len() == 1
    }

    /// Returns a vector of the first `n` atoms of the ACM.
    /// Because of underlying usage of [`atomic`], using [`atoms`] requires that the ACM binding be
    /// declared mutable.
    ///
    /// # Examples
    /// ```
    /// let mut acm = acm::ArithmeticCongruenceMonoid::new(3, 6).unwrap();
    /// assert_eq!(acm.n_atoms(5, acm.a()), [3, 15, 21, 33, 39]);
    /// ```
    /// [`atomic`]: ./struct.ArithmeticCongruenceMonoid.html#method.atomic
    /// [`atoms`]: ./struct.ArithmeticCongruenceMonoid.html#method.atoms
    pub fn n_atoms(&mut self, n: u64, s: u64) -> Vec<u64> {
        let s = self.nearest(s);
        (s..)
            .step_by(self.b as usize)
            .filter(|x| self.atomic(*x))
            .take(n as usize)
            .collect()
    }

    /// Returns a vector of the density (distance between) the first `n` atoms of the ACM.
    /// Because of underlying usage of [`atoms`], using [`atom_density`] requires that the ACM
    /// binding be declared mutable.
    ///
    /// # Examples
    /// ```
    /// let mut acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.atom_density(10, acm.a()), [4, 4, 4, 4, 8, 4, 4, 4, 8]);
    /// ```
    /// [`atoms`]: ./struct.ArithmeticCongruenceMonoid.html#method.atoms
    /// [`atom_density`]: ./struct.ArithmeticCongruenceMonoid.html#method.atom_density
    pub fn atomic_density(&mut self, n: u64, s: u64) -> Vec<u64> {
        let atoms = self.n_atoms(n, s);
        atoms
            .iter()
            .zip(atoms.iter().skip(1))
            .map(|(a1, a2)| a2 - a1)
            .collect::<Vec<u64>>()
    }
}
