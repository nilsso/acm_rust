pub mod lib;
pub mod prime_factors;

use prime_factors::prime_factors;

fn main() {
    println!("{:?}", prime_factors(420));
}
