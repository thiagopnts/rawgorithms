
// in-place merge sort implementation
pub fn mergesort<T : Clone + PartialOrd>(a: &mut Vec<T>) {
    let mut aux = a.clone();
    let len = a.len() - 1;
    sort(a, &mut aux, 0, len);
}

fn sort<T : Clone + PartialOrd>(a: &mut Vec<T>, aux: &mut Vec<T>, lo: uint, hi: uint) {
    if hi <= lo { return; }
    let mid = lo + (hi - lo) / 2u;
    sort(a, aux, lo, mid);
    sort(a, aux, mid + 1, hi);
    merge(a, aux, lo, mid, hi);
}

fn merge<T : Clone + PartialOrd>(a: &mut Vec<T>, aux: &mut Vec<T>, lo: uint, mid: uint, hi: uint) {
    for k in range(lo, hi + 1) {
        *aux.get_mut(k) = a.get(k).clone();
    }

    let mut i = lo;
    let mut j = mid + 1;

    for k in range(lo, hi + 1) {
        if i > mid {
            *a.get_mut(k) = aux.get(j).clone();
            j += 1
        } else if j > hi {
            *a.get_mut(k) = aux.get(i).clone();
            i += 1
        } else if aux.get(j) < aux.get(i) {
            *a.get_mut(k) = aux.get(j).clone();
            j += 1
        } else {
            *a.get_mut(k) = aux.get(i).clone();
            i += 1
        }
    }
}


