# acm_rust

## Tests
- [ ] ACM tests
- [x] [Prime factor tests](https://github.com/nilsso/acm-rust/blob/master/tests/prime_factors.rs)
- [x] [Divisor tests](https://github.com/nilsso/acm-rust/blob/master/tests/divisors.rs)

To run all tests:
```bash
cargo test
```

## Documentation
To build all of the documentation and open it in your browser:
```bash
cargo doc --all --open
```

## Sub-modules

### Prime factors
The `prime_factors` module provides a function `prime_factors` which given an
integer `n` returns a vector of pairs: the integer factor and power (the number
of times it is a factor). For example:
```rust
println!("{:?}", prime_factors::prime_factors(420));
```
Yields:
```
[(2, 2), (3, 1), (5, 1), (7, 1)]
```

### Divisors
(Todo!)
