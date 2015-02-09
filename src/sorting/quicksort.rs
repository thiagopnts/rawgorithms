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
        while a.get(i) < a.get(lo as usize) {
            i += 1;
            if i == hi as usize { break; }
        }

        while a.get(lo as usize) < a.get(j) {
            j -= 1;
            if j == lo as usize { break; }
        }

        if i >= j { break; }
        exch(a, i, j);
    }

    exch(a, lo as usize, j);
    j as int
}

