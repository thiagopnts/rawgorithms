use utils::{exch,shuffle};

pub fn quicksort<T: Clone + PartialOrd>(a: &mut Vec<T>) {
    shuffle(a);
    let len = a.len() - 1;
    sort(a, 0, len as int);
}

fn sort<T: Clone + PartialOrd>(a: &mut Vec<T>, lo: int, hi: int) {
    if hi <= lo { return; }
    let j = partition(a, lo, hi);
    sort(a, lo, j - 1);
    sort(a, j + 1, hi);
}

fn partition<T: Clone + PartialOrd>(a: &mut Vec<T>, lo: int, hi: int) -> int {
    let mut i = lo as usize;
    let mut j = hi as usize;
    loop {
        while a[i] < a[lo as usize] {
            i += 1;
            if i == hi as usize { break; }
        }

        while a[lo as usize] < a[j] {
            j -= 1;
            if j == lo as usize { break; }
        }

        if i >= j { break; }
        exch(a, i, j);
    }

    exch(a, lo as usize, j);
    j as int
}

#[cfg(test)]
mod test {
    use super::quicksort;
    use utils::knuth_shuffle;

    #[test]
    fn test_quicksort() {
        let mut shuffled_array = knuth_shuffle(100);
        quicksort(&mut shuffled_array);

        for value in range(1, 100) {
            assert_eq!(value, shuffled_array[value - 1]);
        }
    }
}

