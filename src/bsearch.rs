use std::cmp::Ordering;

/// `[T]`に対して条件`c`を満たす要素を線形探索し、あれば`true`を返す。
/// ```
/// use crate::toa::bsearch::bsearch;
///
/// let vec: Vec<i32> = vec![-2, -1, 1, 4, 8, 8, 9, 10];
/// let result = bsearch(&vec, |i| 8i32.cmp(i));
/// assert_eq!(result, true);
/// assert_eq!(bsearch(&vec, |i| 100i32.cmp(i)), false);
/// ```
pub fn bsearch<T>(v: &[T], c: impl Fn(&T) -> Ordering) -> bool {
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j {
        let mid = (i + j) / 2;
        match c(&v[mid]) {
            Ordering::Equal => return true,
            Ordering::Less => {
                j = mid - 1;
            }
            Ordering::Greater => {
                i = mid + 1;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bsearch_test1() {
        // let vec: Vec<i32> = vec![1, 4, 8, 9, -1, -2, 8, 10];
        let vec: Vec<i32> = vec![-2, -1, 1, 4, 8, 8, 9, 10];
        fn equal_8 (i: &i32) -> Ordering { 8.cmp(i) }
        assert_eq!(bsearch(&vec, equal_8), true);
    }
    #[test]
    fn bsearch_test2() {
        // let vec: Vec<i32> = vec![1, 4, 8, 9, -1, -2, 8, 10];
        let vec: Vec<i32> = vec![-2, -1, 1, 4, 8, 8, 9, 10];
        assert_eq!(bsearch(&vec, |i| 7i32.cmp(i)), false);
    }
}
