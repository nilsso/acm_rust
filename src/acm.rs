#![feature(trait_alias)]
#![feature(step_trait)]
pub mod divisors;
pub mod factor;
pub mod integers;

use std::cmp::{Eq, PartialOrd};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::{Send, Sync};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

//use common_macros::hash_map;
use failure::Fail;
use num_traits::{One, Pow, Zero};

use divisors::divisors;
//use integers::GCD;

/// Error to encapsulate invalid ACM construction parameters.
#[derive(Fail, Debug)]
#[fail(display = "{} incongruent to {} modulus {}.", _0, _1, _2)]
pub struct ACMError<T: Display + Send + Sync + Debug + 'static>(T, T, T);

pub struct ACMElementIterator<T> {
    _a: T,
    b: T,
    n: T,
}

impl<T> ACMElementIterator<T> {
    pub fn new(a: T, b: T, n: T) -> Self {
        Self { _a: a, b, n }
    }
}

impl<T> Iterator for ACMElementIterator<T>
where
    T: Clone,
    for<'b> T: AddAssign<&'b T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let res = self.n.clone();
        self.n += &self.b;
        Some(res)
    }
}

pub trait TBounds = Zero
    + One
    + AddAssign
    + DivAssign
    + Eq
    + PartialOrd
    + Clone
    + Send
    + Sync
    + Hash
    + Display
    + Debug;

pub trait Ops<A, B> = Add<A, Output = B>
    + Sub<A, Output = B>
    + Mul<A, Output = B>
    + Div<A, Output = B>
    + Rem<A, Output = B>
    + Pow<usize, Output = B>;

#[rustfmt::skip]
pub trait AssignOps<B> = AddAssign<B>
    + SubAssign<B>
    + MulAssign<B>
    + DivAssign<B>
    + RemAssign<B>;

/// Arithmetic congruence monoid implementation.
#[derive(Debug)]
pub struct ArithmeticCongruenceMonoid<T>
where
    T: TBounds + Ops<T, T>,
    for<'a> &'a T: Ops<T, T>,
    for<'b> T: Ops<&'b T, T> + AssignOps<&'b T>,
    for<'a, 'b> &'a T: Ops<&'b T, T>,
{
    a: T,
    b: T,
    factorizations: HashMap<T, Vec<Vec<T>>>,
}

impl<T> ArithmeticCongruenceMonoid<T>
where
    T: TBounds + Ops<T, T>,
    for<'a> &'a T: Ops<T, T>,
    for<'b> T: Ops<&'b T, T> + AssignOps<&'b T>,
    for<'a, 'b> &'a T: Ops<&'b T, T>,
{
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
    pub fn new(a: T, b: T) -> Result<ArithmeticCongruenceMonoid<T>, ACMError<T>> {
        if (&a % &b) == (&a * &a) % &b {
            let mut factorizations = HashMap::new();
            factorizations.insert(T::one(), vec![vec![]]);
            Ok(ArithmeticCongruenceMonoid {
                a: (&a % &b),
                b,
                factorizations,
            })
        } else {
            let c = &a * &a;
            Err(ACMError(a, c, b))
        }
    }

    /// Returns the $a$ component of the ACM.
    pub fn a(&self) -> &T {
        &self.a
    }

    /// Returns the $b$ component of the ACM.
    pub fn b(&self) -> &T {
        &self.b
    }

    /// Returns `true` if `n` is an element of the ACM.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert!( acm.contains(&5));
    /// assert!(!acm.contains(&6));
    /// ```
    pub fn contains(&self, x: &T) -> bool {
        &(x % &self.b) == &self.a
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
    pub fn nearest<U: Into<T>>(&self, s: U) -> T {
        let s: T = s.into();
        if &s >= &self.a {
            let c = &s - &self.a;
            s - c % &self.b
        } else {
            self.a.clone()
        }
    }

    /// Returns an iterator over ACM elements.
    pub fn iter(&self) -> ACMElementIterator<T> {
        self.iter_from(self.a.clone())
    }

    pub fn iter_from(&self, s: T) -> ACMElementIterator<T> {
        ACMElementIterator::new(self.a.clone(), self.b.clone(), self.nearest(s))
    }

    /// Returns the $n$th ACM element.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.ith(0), 1);
    /// assert_eq!(acm.ith(1), 5);
    /// assert_eq!(acm.ith(56), 225);
    /// ```
    pub fn ith<U: Into<T>>(&self, i: U) -> T {
        &self.a + &self.b * i.into()
    }

    /// Get ACM element index of an integer.
    pub fn index(&self, n: T) -> Option<T> {
        if self.contains(&n) {
            Some((n - &self.a) / &self.b + T::one())
        } else {
            None
        }
    }

    /// Returns the ACM element divisors of an integer `n`.
    ///
    /// # Examples
    /// ```
    /// let acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert_eq!(acm.divisors(225), [1, 9, 5, 25, 45, 225]);
    /// ```
    pub fn divisors(&self, n: T) -> Vec<T> {
        divisors(n)
            .into_iter()
            .filter(|x| self.contains(x))
            .collect()
    }

    /// Returns a reference to the vector of ACM atom factorizations of an integer `n`.
    /// If `n` is not an element of the ACM then the vector will be empty.
    /// Because factorization results are stored internally to the ACM in order to reduce
    /// computational costs, using [`factor`] requires that the ACM binding be declared mutable.
    ///
    /// # Examples
    /// ```
    /// let mut acm = acm::ArithmeticCongruenceMonoid::new(3, 6).unwrap();
    /// assert_eq!(acm.factor(1),   &[[]]);
    /// assert_eq!(acm.factor(2),   &[[]; 0]);
    /// assert_eq!(acm.factor(3),   &[[3]]);
    /// assert_eq!(acm.factor(9),   &[[3, 3]]);
    /// assert_eq!(acm.factor(225), &[[15, 15], [3, 75]]);
    /// ```
    /// [`factor`]: ./struct.ArithmeticCongruenceMonoid.html#methods.factor
    pub fn factor<U: Into<T>>(&mut self, n: U) -> &Vec<Vec<T>> {
        let n: T = n.into();

        // TODO: Further optimize
        if self.factorizations.contains_key(&n) {
            return self.factorizations.get(&n).unwrap();
        }

        self.factorizations.insert(n.clone(), vec![]);

        if self.contains(&n) {
            let n_ds = self.divisors(n.clone());
            for (d, q) in n_ds
                .iter()
                .take(n_ds.len() - 1)
                .map(|d| (d.clone(), &n / d))
                // Considering squaring both sides (problem is with overflow)
                //.filter(|(d, q)| *d >= ((*q as f32).sqrt() as u64))
                .filter(|(d, q)| &(d * d) >= q)
            {
                if let Some(d_fs) = self.factor(d.clone()).first() {
                    if d_fs.len() == 1 {
                        for mut q_f in self.factor(q).clone().into_iter() {
                            if q_f.is_empty() || &d >= q_f.last().unwrap() {
                                q_f.push(d.clone());
                                self.factorizations.get_mut(&n).unwrap().push(q_f);
                            }
                        }
                    }
                }
            }
            if self.factorizations.get(&n).unwrap().is_empty() {
                self.factorizations
                    .get_mut(&n)
                    .unwrap()
                    .push(vec![n.clone()]);
            }
        }
        self.factorizations.get(&n).unwrap()
    }

    /// Returns `true` if `n` is atomic under the ACM (is an ACM element, and cannot be expressed
    /// as a product of smaller ACM atoms).
    /// Because of underlying usage of [`factor`], using [`atomic`] requires that the ACM binding be
    /// declared mutable.
    ///
    /// # Examples
    /// ```
    /// let mut acm = acm::ArithmeticCongruenceMonoid::new(1, 4).unwrap();
    /// assert!( acm.contains(&5)  &&  acm.atomic(&5));
    /// assert!(!acm.contains(&15) && !acm.atomic(&15));
    /// assert!( acm.contains(&25) && !acm.atomic(&25));
    /// ```
    /// [`factor`]: ./struct.ArithmeticCongruenceMonoid.html#method.factor
    /// [`atomic`]: ./struct.ArithmeticCongruenceMonoid.html#method.atomic
    pub fn atomic(&mut self, n: &T) -> bool {
        if !self.contains(&n) {
            return false;
        }
        let n_fs = self.factor(n.clone());
        n_fs.len() == 1 && n_fs.first().unwrap().len() <= 1
    }
}
