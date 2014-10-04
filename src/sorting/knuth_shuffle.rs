
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


