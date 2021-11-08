/// `[T]`に対して条件`c`を満たす要素を線形探索し、あれば`true`を返す。
/// ```
/// use crate::toa::lsearch::lsearch;
///
/// let vec: Vec<i32> = vec![1, 4, 8, 9, -1, -2, 8, 10];
/// let result = lsearch(&vec, |i| *i == 8i32);
/// assert_eq!(result, true);
/// ```
pub fn lsearch<T>(v: &[T], c: impl Fn(&T) -> bool) -> bool {
    let mut i = 0;
    while i < v.len() {
        if c(&v[i]) { return true }
        i += 1;
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lsearch_test1() {
        let vec: Vec<i32> = vec![1, 4, 8, 9, -1, -2, 8, 10];
        fn equal_8 (i: &i32) -> bool { *i == 8 }
        assert_eq!(lsearch(&vec, equal_8), true);
    }
    #[test]
    fn lsearch_test2() {
        let vec: Vec<i32> = vec![1, 4, 8, 9, -1, -2, 8, 10];
        assert_eq!(lsearch(&vec, |i| *i == 7i32), false);
    }
}
