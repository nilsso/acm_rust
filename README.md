Arithmetic Congruence Monoid
============================

Implements [arithmetic congruence monoids][math-acm] (ACM) in Rust.

[math-acm]: http://faculty.fairfield.edu/pbaginski/Papers/SubmittedACMSurvey%20RevisedReferee%2001.20.2013.pdf

In a nutshell, an ACM is an arithmetic progression which possesses a multiplicative structure,
and specifically is the monoid:
<center>
<img src="./tex/acm.png" height="24pt">
</center>

With <img src="./tex/hilbert-a.png" height="14pt">
and <img src="./tex/hilbert-b.png" height="14pt">
we get a Hilbert monoid:
<center>
<img src="./tex/hilbert.png" height="24pt">
</center>

## CLI
First build:
```
cargo b
```
Which provides `acm_rust` with subcommands to test the main ACM module
and the divisors/factorize submodules.
```
> ./target/debug/acm_rust ACM 3 6 F 225 -v
ACM(3,6) prime power factorization of 225 : [[15, 15], [3, 75]]
```

<!--
   -
   -## Submodules
   -
   -### Prime power factorization
   -The [`factorize`][factorize] submodule provides a function `factorize` which
   -given an integer returns a vector of pairs of prime integer factors and powers
   -(the number of times it is a factor). For example:
   -```rust
   -assert_eq!(prime_factors::factorize(420), vec![(2, 2), (3, 1), (5, 1), (7, 1)]);
   -```
   -
   -[factorize]: https://github.com/nilsso/acm_rust/blob/master/src/factorize.rs
   -
   -### Divisors
   -The [`divisors`][divisors] submodule provides a function `divisors` which given an
   -integer returns a vector of integer divisors.
   -For example:
   -```rust
   -assert_eq!(divisors::divisors(18), vec![1, 2, 3, 9, 6, 18]);
   -```
   -
   -[divisors]: https://github.com/nilsso/acm_rust/blob/master/src/divisors.rs
   -->

## Documentation
Build all documentation and open in your browser:
```
cargo doc --all --open
```

## Tests
```
cargo t
```
