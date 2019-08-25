/// Returns the prime power factorization of an integer.
///
/// # Examples
/// ```
/// assert_eq!(acm::factorize(120), [(2, 3), (3, 1), (5, 1)]);
/// ```
pub fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut pfs: Vec<(u64, u32)> = Vec::new();
    let mut d = 2_u64;
    while n > 1 {
        while n % d != 0 {
            d += 1;
        }
        let mut q = n / d;
        let mut i = 1_u32;
        while q % d == 0 {
            q /= d;
            i += 1;
        }
        pfs.push((d, i));
        n /= d.pow(i);
    }
    pfs
}
