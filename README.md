# Arithmetic Congruence Monoid (Rust)

Implements [arithmetic congruence monoids][math-acm] (ACM) in Rust.

[math-acm]: http://faculty.fairfield.edu/pbaginski/Papers/SubmittedACMSurvey%20RevisedReferee%2001.20.2013.pdf

In a nutshell, an ACM is an arithmetic progression which possesses a multiplicative structure,
and specifically is the monoid:
![ACM monoid](./tex/acm.png)

## CLI

## Module

## Submodules

### Prime power factorization
The `prime_factors` submodule provides a function `factorize` which given an
integer returns a vector of pairs of prime integer factors and powers (the number
of times it is a factor).
For example:
```rust
assert_eq!(prime_factors::factorize(420), vec![(2, 2), (3, 1), (5, 1), (7, 1)]);
```

### Divisors
The `divisors` submodule provides a function `divisors` which given an
integer returns a vector of its integer divisors.
For example:
```rust
assert_eq!(divisors::divisors(18), vec![1, 2, 3, 9, 6, 18]);

## Documentation
(Not much as of yet.)
To build all of the documentation and open it in your browser:
```bash
cargo doc --all --open
```

## Tests
- [x] [Prime factor tests](https://github.com/nilsso/acm-rust/blob/master/tests/prime_factors.rs)
- [x] [Divisor tests](https://github.com/nilsso/acm-rust/blob/master/tests/divisors.rs)
- [x] [ACM tests](https://github.com/nilsso/acm_rust/blob/master/tests/acm.rs)

To run all tests:
```bash
cargo test
```

## Notes
- Clap example [yml][clap-yml] / [rs][clap-rs]

[clap-yml]: https://github.com/clap-rs/clap/blob/master/examples/17_yaml.yml
[clap-rs]: https://github.com/clap-rs/clap/blob/master/examples/17_yaml.rs
