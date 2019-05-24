Arithmetic Congruence Monoid
============================

[![Latest version](https://img.shields.io/crates/v/acm.svg)](https://crates.io/crates/acm)
[![Documentation](https://docs.rs/acm/badge.svg)](https://docs.rs/acm)

Implements [arithmetic congruence monoids][math-acm] (ACM) in Rust.

[math-acm]: http://faculty.fairfield.edu/pbaginski/Papers/SubmittedACMSurvey%20RevisedReferee%2001.20.2013.pdf

In a nutshell, an ACM is an arithmetic progression which possesses a multiplicative structure,
and specifically is the monoid:
<center>
<img src="img//2.png" height=24pt>
</center>

With <img src="img//0.png" height=14pt> and <img src="img//1.png" height=14pt> we get a Hilbert monoid:
<center>
<img src="img//3.png" height=24pt>
</center>

Over an ACM, we can factor integers into elements of the ACM. This is similar to
simple prime factorization except that each factor must be an element of the
ACM. We call elements which cannot be expressed as the product of smaller ACM
elements *atoms*.

Finally, the purpose of this library is to study the atomic density of different
ACMs, that is, the distance between atoms. In certain ACMs the atomic density is
provably constant throughout, but in others it is unknown, not dissimilar to the
density, or lack thereof, of prime integers in the set of all integers.

## CLI
First build:
```
cargo b
```
Provided is a CLI program `acm-cli` with subcommands to test the main ACM
module and the divisors/factorize submodules.
```
> ./target/debug/acm-cli acm 3 6 factorize 225
[[15, 15], [3, 75]]
```

For full usage lists, try it, and any subcommand, with the `-h` flag:
```
> ./target/debug/acm-cli -h
acm-cli 0.1
nilsso <nilso@enosis.net>

USAGE:
    acm-cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    acm          ArithmeticCongruenceMonoid subcommand
    divisors     Integer divisors subcommand
    factorize    Integer factorization subcommand
    help         Prints this message or the help of the given subcommand(s)
```

## Documentation
Build all documentation locall and open in your browser:
```
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps --open
```

## Tests
```
cargo t
```
