use utils::exch;

pub fn selection_sort<T: Clone + PartialOrd>(a: &mut Vec<T>) {
    let mut min;
    let len = a.len();
    for i in range(0, len) {
        min = i;
        for j in range(i + 1, len) {
            if a[j] < a[min] {
                min = j;
            }
        }
        exch(a, i, min);
    }
}
