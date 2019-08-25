extern crate acm;
use acm::divisors;

fn helper(n: u64, ans: Vec<u64>) {
    let mut divisors = divisors(n);
    divisors.sort();
    assert_eq!(divisors, ans);
}

#[test]
fn divisors_of_2() {
    helper(2, vec![1, 2]);
}

#[test]
fn divisors_of_5() {
    helper(5, vec![1, 5]);
}

#[test]
fn divisors_of_4() {
    helper(4, vec![1, 2, 4]);
}

#[test]
fn divisors_of_12() {
    helper(12, vec![1, 2, 3, 4, 6, 12]);
}

#[test]
fn divisors_of_60() {
    helper(60, vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);
}

#[test]
fn divisors_of_420() {
    helper(
        420,
        vec![
            1, 2, 3, 4, 5, 6, 7, 10, 12, 14, 15, 20, 21, 28, 30, 35, 42, 60, 70, 84, 105, 140, 210,
            420,
        ],
    );
}
