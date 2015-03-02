use utils::exch;

pub fn shellsort<T: Clone + PartialOrd>(a: &mut Vec<T>) {
    let len = a.len() as int;
    let mut h = 1i;
    while h < (len / 3) {
        h = 3 * h + 1;
    }

    while h >= 1 {
        for i in range(h, len) {
            let mut j: int = i;
            while j >= h && a[j as usize] < a[(j - h) as usize] {
                exch(a, j as usize, (j - h) as usize);
                j -= h;
            }
        }
        h = h / 3;
    }
}

#[cfg(test)]
mod test {
    use super::shellsort;
    use utils::knuth_shuffle;

    #[test]
    fn test_shellsort() {
        let mut shuffled_array = knuth_shuffle(100);
        shellsort(&mut shuffled_array);

        for value in range(1, 100) {
            assert_eq!(value, shuffled_array[value - 1]);
        }
    }
}
