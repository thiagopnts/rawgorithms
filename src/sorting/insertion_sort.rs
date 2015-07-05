use utils::exch;

pub fn insertion_sort<T: Clone + PartialOrd>(a: &mut Vec<T>) {
    let len = a.len();

    for i in 0..len {
        for j in (1..i + 1).rev() {
            if a.get(j) < a.get(j - 1) {
                exch(a, j, j - 1);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::insertion_sort;
    use utils::knuth_shuffle;

    #[test]
    fn test_insertion_sort() {
        let mut shuffled_array = knuth_shuffle(100);
        insertion_sort(&mut shuffled_array);

        for value in 1..100 {
            assert!(value == shuffled_array[value - 1]);
        }
    }
}
