use utils::exch;

pub fn insertion_sort<T: Clone + PartialOrd>(a: &mut Vec<T>) {
    let len = a.len();

    for i in range(0, len) {
        for j in range(1, i + 1).rev() {
            if a.get(j) < a.get(j - 1) {
                exch(a, j, j - 1);
            } else {
                break;
            }
        }
    }
}
