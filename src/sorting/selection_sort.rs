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

#[cfg(test)]
mod test {
    use super::selection_sort;
    use utils::knuth_shuffle;

    #[test]
    fn test_selection_sort() {
        let mut shuffled_array = knuth_shuffle(100);
        selection_sort(&mut shuffled_array);

        for value in range(1, 100) {
            assert_eq!(value, shuffled_array[value - 1]);
        }
    }
}
