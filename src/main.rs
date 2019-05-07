use std::collections::BTreeSet;

fn main() {
    let mut s = BTreeSet::new();

    s.insert(1);
    s.insert(1);
    s.insert(2);

    println!("{:?}", s);
}
