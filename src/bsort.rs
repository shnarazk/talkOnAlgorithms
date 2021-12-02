///
///```
/// use crate::toa::bsort::bsort;
///
/// let mut vec: Vec<i32> = vec![-1, 4, 8, 1, 10, 8, -2, 9];
/// let mut ans = vec.clone();
/// bsort(&mut vec);
/// ans.sort();
/// assert_eq!(vec, ans);
///```
pub fn bsort<T: Ord>(v: &mut [T]) {
    for i in 0..v.len() {
        for j in ((i + 1)..v.len()).rev() {
            if v[j] < v[j - 1] {
                v.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::SliceRandom;

    #[test]
    fn lsearch_test_1000() {
        let mut seed = rand::thread_rng();
        let mut vec: Vec<i32> = (-10i32..90).collect();
        for _ in 1..1000 {
            vec.shuffle(&mut seed);
            let mut copied = vec.clone();
            bsort(&mut vec);
            copied.sort();
            assert_eq!(vec, copied);
        }
    }
}
