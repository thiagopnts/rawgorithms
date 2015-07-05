// in-place merge sort implementation
pub fn mergesort<T : Clone + PartialOrd>(a: &mut Vec<T>) {
    let mut aux = a.clone();
    let len = a.len() - 1;
    sort(a, &mut aux, 0, len);
}

fn sort<T : Clone + PartialOrd>(a: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, hi: usize) {
    if hi <= lo { return; }
    let mid = lo + (hi - lo) / 2;
    sort(a, aux, lo, mid);
    sort(a, aux, mid + 1, hi);
    merge(a, aux, lo, mid, hi);
}

fn merge<T : Clone + PartialOrd>(a: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, mid: usize, hi: usize) {
    for k in lo..hi + 1 {
        aux[k] = a[k].clone();
    }

    let mut i = lo;
    let mut j = mid + 1;

    for k in lo..hi + 1 {
        if i > mid {
            a[k] = aux[j].clone();
            j += 1
        } else if j > hi {
            a[k] = aux[i].clone();
            i += 1
        } else if aux[j] < aux[i] {
            a[k] = aux[j].clone();
            j += 1
        } else {
            a[k] = aux[i].clone();
            i += 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::mergesort;
    use utils::knuth_shuffle;

    #[test]
    fn test_mergesort() {
        let mut shuffled_array = knuth_shuffle(100);
        mergesort(&mut shuffled_array);

        for value in 1..100 {
            assert!(value == shuffled_array[value - 1]);
        }
    }
}

