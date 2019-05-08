use common_macros::b_tree_set;

extern crate acm_rust;
use acm_rust::divisors::divisors;

#[test]
fn divisors_of_2() {
    assert_eq!(divisors(2), b_tree_set!{1,2});
}

#[test]
fn divisors_of_5() {
    assert_eq!(divisors(5), b_tree_set!{1,5});
}

#[test]
fn divisors_of_4() {
    assert_eq!(divisors(4), b_tree_set!{1,2,4});
}

#[test]
fn divisors_of_12() {
    assert_eq!(divisors(12), b_tree_set!{1,2,3,4,6,12});
}

#[test]
fn divisors_of_60() {
    assert_eq!(divisors(60), b_tree_set!{1,2,3,4,5,6,10,12,15,20,30,60});
}

#[test]
fn divisors_of_420() {
    assert_eq!(divisors(420), b_tree_set!{
        1,2,3,4,5,6,7,10,12,14,15,20,21,28,30,35,42,60,70,84,105,140,210,420
    });
}



