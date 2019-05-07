/** Prime factors
 */
pub fn prime_factors(mut n: i32) -> Vec<(i32,i32)> {
    let mut pfs: Vec<(i32,i32)> = Vec::new();
    let mut d = 2;
    let m = f32::sqrt(n as f32) as i32;
    while n > 1 {
        while n % d != 0 {
            d += 1;
        }
        let mut dd = n/d;
        let mut i: u32 = 1;
        while dd % d == 0 {
            dd /= d;
            i += 1;
        }
        pfs.push((d, i as i32));
        n /= d.pow(i);
        if d == m {
            d = n;
        }
    }

    pfs
}

#[cfg(test)]
mod tests {
    use super::prime_factors;

    #[test]
    fn prime_factors_of_2() {
        assert_eq!(prime_factors(2), vec![(2,1)]);
    }

    #[test]
    fn prime_factors_of_4() {
        assert_eq!(prime_factors(4), vec![(2,2)]);
    }

    #[test]
    fn prime_factors_of_5() {
        assert_eq!(prime_factors(5), vec![(5,1)]);
    }

    #[test]
    fn prime_factors_of_12() {
        assert_eq!(prime_factors(12), vec![(2,2),(3,1)]);
    }

    #[test]
    fn prime_factors_of_60() {
        assert_eq!(prime_factors(60), vec![(2,2),(3,1),(5,1)]);
    }

    #[test]
    fn prime_factors_of_420() {
        assert_eq!(prime_factors(420), vec![(2,2),(3,1),(5,1),(7,1)]);
    }
}
