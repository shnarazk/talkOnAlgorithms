///
///```
/// use crate::toa::bsort::bsort;
///
/// let mut vec: Vec<i32> = vec![-1, 4, 8, 1, 10, 8, -2, 9];
/// let  sorted: Vec<i32> = vec![-2, -1, 1, 4, 8, 8, 9, 10];
/// bsort(&mut vec);
/// assert_eq!(vec, sorted);
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
