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
