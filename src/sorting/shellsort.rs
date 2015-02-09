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
            while j >= h && a.get(j as usize) < a.get((j - h) as usize) {
                exch(a, j as usize, (j - h) as usize);
                j -= h;
            }
        }
        h = h / 3;
    }
}
