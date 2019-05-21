pub fn prime_factors(mut n: u32) -> Vec<(u32, u32)> {
    let mut pfs: Vec<(u32, u32)> = Vec::new();
    let mut d = 2;
    while n > 1 {
        while n % d != 0 {
            d += 1;
        }
        let mut q = n / d;
        let mut i: u32 = 1;
        while q % d == 0 {
            q /= d;
            i += 1;
        }
        pfs.push((d, i));
        n /= d.pow(i);
    }
    pfs
}
