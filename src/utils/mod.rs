extern crate rand;

use std::vec::Vec;

use self::rand::random;

// linear time shuffling algorithm
pub fn knuth_shuffle(n: uint) -> Vec<uint> {
    let mut vec: Vec<uint> = (0 .. n).map(|i| i + 1).collect();
    for i in range(0u, n) {
        let r = random::<uint>() % (i + 1);
        let value: uint = vec[i];
        vec[i] = vec[r];
        vec[r] = value;
    }
    vec
}

pub fn shuffle<T>(v: &mut Vec<T>) where T: Clone, T: PartialOrd {
    for i in range(0u, v.len()) {
        let r = random::<uint>() % (i + 1);
        exch(v, i, r);
    }
}

pub fn exch<T: Clone + PartialOrd>(a: &mut Vec<T>, i: uint, j: uint) {
    let from = a[i].clone();
    let to = a[j].clone();
    a[i] = to;
    a[j] = from;
}

pub fn is_sorted<T: PartialOrd>(a: &Vec<T>) -> bool {
    for i in range(1, a.len()) {
        if a[i - 1] > a[i] { return false; }
    }
    true
}


