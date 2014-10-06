use utils::exch;

pub fn quicksort<T: Clone + PartialOrd>(a: &mut Vec<T>) {
    // shuffle array
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
    let mut i = lo as uint;
    let mut j = hi as uint;
    loop {
        while a.get(i) < a.get(lo as uint) {
            i += 1;
            if i == hi as uint { break; }
        }

        while a.get(lo as uint) < a.get(j) {
            j -= 1;
            if j == lo as uint { break; }
        }

        if i >= j { break; }
        exch(a, i, j);
    }

    exch(a, lo as uint, j);
    j as int
}

