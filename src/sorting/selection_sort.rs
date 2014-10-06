use utils::exch;

pub fn selection_sort<T: Clone + PartialOrd>(a: &mut Vec<T>) {
    let len = a.len();
    let mut min = -1;
    for i in range(0, len) {
        min = i;
        for j in range(i + 1, len) {
            if a.get(j) < a.get(min) {
                min = j;
            }
        }
        exch(a, i, min);
    }
}
