mod divisors;
mod factorize;

// Provide submodule functions as crate functions
pub use divisors::divisors;
pub use factorize::factorize;

#[macro_use]
extern crate failure;

use common_macros::hash_map;
use std::collections::HashMap;

/// Error to encapsulate invalid ACM construction parameters.
#[derive(Fail, Debug)]
#[fail(display = "{} incongruent to {} modulus {}.", _0, _1, _2)]
pub struct ACMError(u32, u32, u32);

/// Arithmetic congruence monoid implementation.
#[derive(Debug)]
pub struct ArithmeticCongruenceMonoid {
    a: u32,
    b: u32,
    factorizations: HashMap<u32, Vec<Vec<u32>>>,
}

type ACM = ArithmeticCongruenceMonoid;

impl ArithmeticCongruenceMonoid {
    /// Attempts to construct a new ACM with components `a` and `b`,
    /// requiring that `a % b == a.pow(2) % b`.
    ///
    /// # Examples
    /// ```
    /// // A valid ACM (1 % 4 == 1 == 1*1 % 4)
    /// assert!(acm::ArithmeticCongruenceMonoid::new(1, 4).is_ok());
    ///
    /// // An invalid ACM (2 % 4 == 2 != 0 == 2*2 % 4)
    /// assert!(acm::ArithmeticCongruenceMonoid::new(2, 4).is_err());
    /// ```
    pub fn new(a: u32, b: u32) -> Result<ACM, ACMError> {
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

    /// Returns the `a` component of the ACM.
    pub fn a(&self) -> u32 {
        self.a
    }

    /// Returns the `b` component of the ACM.
    pub fn b(&self) -> u32 {
        self.b
    }

    /// Returns the nearst ACM element equal to or below `n`.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.element(5), 5);
    /// assert_eq!(acm.element(6), 5);
    /// ```
    pub fn element(&self, n: u32) -> u32 {
        n - (n - self.a) % self.b
    }

    /// Generate `n` ACM elements starting at nearest element below or equal to `s`.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.elements(5, 1), [1, 5, 9, 13, 17]);
    /// ```
    pub fn elements(&self, n: u32, s: u32) -> Vec<u32> {
        let s = self.element(s);
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
    pub fn contains(&self, n: u32) -> bool {
        n == 1 || n % self.b == self.a
    }

    /// Returns the ACM element divisors of an integer `n`.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.divisors(225), [1, 9, 5, 25, 45, 225]);
    /// ```
    pub fn divisors(&self, n: u32) -> Vec<u32> {
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
    pub fn factorize(&mut self, n: u32) -> &Vec<Vec<u32>> {
        // TODO: Further optimize
        if self.factorizations.contains_key(&n) {
            return self.factorizations.get(&n).unwrap();
        }

        self.factorizations.insert(n, vec![]);

        if self.contains(n) {
            let n_ds = self.divisors(n);
            for (d, q) in n_ds[1..n_ds.len()-1]
                .iter()
                .map(|d| (*d, n / d))
                // Considering squaring both sides (problem is with overflow)
                .filter(|(d, q)| *d >= ((*q as f32).sqrt() as u32))
                //.filter(|(d, q)| *d <= *q)
            {
                //println!("({}) d={} q={} A", n, d, q);
                if let Some(d_fs) = self.factorize(d).first() {
                    //println!("({}) d={} q={} B", n, d, q);
                    if d_fs.len() == 1 {
                        //println!("({}) d={} q={} C", n, d, q);
                        for mut q_f in self.factorize(q).clone().into_iter() {
                            if q_f.is_empty() || &d >= q_f.last().unwrap() {
                                //println!("({}) d={} q={} D", n, d, q);
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
    pub fn atomic(&mut self, n: u32) -> bool {
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
    /// assert_eq!(acm.atoms(5, acm.a()), [3, 15, 21, 33, 39]);
    /// ```
    /// [`atomic`]: ./struct.ArithmeticCongruenceMonoid.html#method.atomic
    /// [`atoms`]: ./struct.ArithmeticCongruenceMonoid.html#method.atoms
    pub fn atoms(&mut self, n: u32, s: u32) -> Vec<u32> {
        let s = self.element(s);
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
    pub fn atom_density(&mut self, n: u32, s: u32) -> Vec<u32> {
        let s = self.element(s);
        let atoms = self.atoms(n, s);
        atoms.iter()
            .zip(atoms.iter().skip(1))
            .map(|(a1, a2)| a2 - a1)
            .collect::<Vec<u32>>()
    }
}
