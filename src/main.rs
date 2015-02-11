#![feature(int_uint)]
mod sorting;
mod data_structures;
mod utils;


fn main() {
    let mut a = utils::knuth_shuffle(10);
    sorting::heapsort(&mut a);
}
