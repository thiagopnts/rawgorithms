mod sorting;
mod data_structures;
mod utils;

fn main() {
    let mut a = utils::knuth_shuffle(10);
    println!("{} sorted: {}", a, utils::is_sorted(&a));
    sorting::quicksort(&mut a);
    println!("{} sorted: {}", a, utils::is_sorted(&a));
    let mut a = 1u;
    unsafe {
    let mut b = &mut a;
        println!("a: {}, b: {} ", a, b);
    }
}
