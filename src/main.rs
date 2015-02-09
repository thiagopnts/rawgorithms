#![feature(int_uint)]
mod sorting;
mod data_structures;
mod utils;


fn main() {
    let mut a = utils::knuth_shuffle(10);
    println!("{} sorted: {}", a, utils::is_sorted(&a));
    sorting::heapsort(&mut a);
    println!("{} sorted: {}", a, utils::is_sorted(&a));
    let mut q = data_structures::QuickFind::new(1);
}
