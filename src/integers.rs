pub trait GCD {
    // Extended Euclidean algorithm
    fn gcd(self, other: Self) -> Self;
}

impl<T: Into<i32> + From<i32>> GCD for T {
    fn gcd(self, other: Self) -> Self {
        let a: i32 = self.into();
        let b: i32 = other.into();
        let mut prev = [a, 1, 0];
        let mut curr = [b, 0, 1];
        while curr[0] != 0 {
            let q = prev[0] / curr[0];
            for i in 0..2 {
                let temp = curr[i];
                curr[i] = prev[i] - q * temp;
                prev[i] = temp;
            }
        }
        prev[0].into()
    }
}
