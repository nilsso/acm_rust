use crate::integers::GCD;

pub fn atomicity_sieve(a: i32, b: i32, n: usize) -> Vec<bool> {
    let mut sieve = vec![true; n];
    let offset = a / a.gcd(b);
    let s = if a != 1 { 0 } else { 1 };
    let e = (n as f32 / a as f32) as usize;
    for i in s..e {
        if sieve[i] {
            let x = a + i as i32 * b;
            for e in sieve
                .iter_mut()
                .skip((i as i32 * a + offset) as usize)
                .step_by(x as usize)
            {
                *e = false;
            }
        }
    }
    sieve
}

pub fn atoms_in_n(a: i32, b: i32, n: usize) -> Vec<i32> {
    atomicity_sieve(a, b, n)
        .into_iter()
        .enumerate()
        .filter_map(|(i, is_atom)| is_atom.then_some(a + i as i32 * b))
        .collect()
}

pub fn reducibles_in_n(a: i32, b: i32, n: usize) -> Vec<i32> {
    atomicity_sieve(a, b, n)
        .into_iter()
        .enumerate()
        .filter_map(|(i, is_atom)| (!is_atom).then_some(a + i as i32 * b))
        .collect()
}
