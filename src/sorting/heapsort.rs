use utils::exch;
// heapsort works for one-based indexed arrays so it will ignore the first element of the
// given array
pub fn heapsort<T>(array: &mut Vec<T>) where T: Clone + PartialOrd {
    let mut len = array.len() - 1;
    let mut k = len / 2;
    while k >= 1 {
        sink(array, k, len);
        k -= 1
    }

    while len > 1 {
        exch(array, 1, len);
        len -= 1;
        sink(array, 1, len);
    }
}

fn sink<T>(array: &mut Vec<T>, k: usize, n: usize) where T: Clone + PartialOrd {
    let mut i = k;
    while (2 * i) <= n {
        let mut j = 2 * i;
        if j < n && array[j] < array[j + 1] {
            j += 1
        }
        if !(array[i] < array[j]) {
            break;
        }
        exch(array, i, j);
        i = j
    }
}

#[cfg(test)]
mod test {
    use super::heapsort;

    #[test]
    fn test_heapsort() {
        // heapsort works only for 1 index-based arrays, so it will ignore the first element
        let mut shuffled_array = vec![1, 2, 1, 3, 5, 4, 6, 8, 9, 7];

        heapsort(&mut shuffled_array);

        for value in range(1, 10) {
            assert!(value == shuffled_array[value]);
        }
    }
}
