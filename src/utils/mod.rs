#![feature(int_uint)]
use std::vec::Vec;
use std::rand::random;

// linear time shuffling algorithm
pub fn knuth_shuffle(n: uint) -> Vec<uint> {
    let mut vec = Vec::from_fn(n, |i| i + 1);
    for i in range(0u, n) {
        let r = random::<uint>() % (i + 1);
        let value = vec[i];
        *vec.get_mut(i) = vec[r];
        *vec.get_mut(r) = value;
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
    let from = a.get(i).clone();
    let to = a.get(j).clone();
    *a.get_mut(i) = to;
    *a.get_mut(j) = from;
}

pub fn is_sorted<T: PartialOrd>(a: &Vec<T>) -> bool {
    for i in range(1, a.len()) {
        if a.get(i - 1) > a.get(i) { return false; }
    }
    true
}


