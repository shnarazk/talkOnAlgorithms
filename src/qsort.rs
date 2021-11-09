use std::fmt::Debug;

///
///```
/// use crate::toa::qsort::qsort;
///
/// let mut vec: Vec<i32> = vec![-1, 4, 8, 1, 10, 8, -2, 9];
/// let  sorted: Vec<i32> = vec![-2, -1, 1, 4, 8, 8, 9, 10];
/// qsort(&mut vec);
/// assert_eq!(vec, sorted);
/// let mut vec: Vec<i32> = vec![-1, 4, 8, 1, 10, 8, -2];
/// let  sorted: Vec<i32> = vec![-2, -1, 1, 4, 8, 8, 10];
/// qsort(&mut vec);
/// assert_eq!(vec, sorted);
/// let mut vec: Vec<i32> = vec![1, 1];
/// let  sorted: Vec<i32> = vec![1, 1];
/// qsort(&mut vec);
/// assert_eq!(vec, sorted);
/// let mut vec: Vec<i32> = vec![1];
/// let  sorted: Vec<i32> = vec![1];
/// qsort(&mut vec);
/// assert_eq!(vec, sorted);
/// let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
/// let  sorted: Vec<i32> = vec![1, 2, 3, 4, 5];
/// qsort(&mut vec);
/// assert_eq!(vec, sorted);
/// let mut vec: Vec<i32> = vec![5, 4, 3, 2, 1];
/// let  sorted: Vec<i32> = vec![1, 2, 3, 4, 5];
/// qsort(&mut vec);
/// assert_eq!(vec, sorted);
/// let mut vec: Vec<i32> = vec![1 ,2, 3, 1, 2, 3, 4, 5, 1, 2, 3];
/// let  sorted: Vec<i32> = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 5];
/// qsort(&mut vec);
/// assert_eq!(vec, sorted);
///```
pub fn qsort<T: Ord + Clone + Debug>(v: &mut [T]) {
    fn sort_range<T: Ord + Debug>(v: &mut [T], left: usize, right: usize) {
        if left >= right {
            return;
        }
        if left + 1 == right {
            if v[right] < v[left] {
                v.swap(left, right);
            }
            return;
        }
        v.swap((left + right) / 2, left);
        let (mid, mut i, mut j) = (left, left + 1, right);
        while i <= j {
            println!(
                "-args/{:?} {:?} | {:?} | {:?}",
                &v[mid],
                &v[..left],
                &v[left..=right],
                &v[right + 1..]
            );
            while v[i] <= v[mid] && i < right {
                i += 1;
            }
            println!(
                "-head/{:?} {:?} | {:?} i={}, j={} | {:?}",
                v[mid],
                &v[..left],
                &v[left..=i],
                i,
                j,
                &v[right + 1..]
            );
            while v[mid] < v[j] && left < j {
                j -= 1;
            }
            println!(
                "-tail/{:?} {:?} | {:?} i={}, j={} | {:?}",
                v[mid],
                &v[..left],
                &v[j..=right],
                i,
                j,
                &v[right + 1..]
            );
            if j <= i {
                if i == left + 1 {
                    println!(
                        "shift</{:?} {:?} | {:?} {:?} | {:?}",
                        &v[mid],
                        &v[..left],
                        &v[left..=left],
                        &v[left + 1..=right],
                        &v[right + 1..]
                    );
                    return sort_range(v, left + 1, right);
                }
                if j == right {
                    v.swap(mid, right);
                    println!(
                        "shift</{:?} {:?} | {:?} {:?} | {:?}",
                        &v[mid],
                        &v[..left],
                        &v[left..right],
                        &v[right..=right],
                        &v[right + 1..]
                    );
                    return sort_range(v, left, right - 1);
                }
                break;
            }
            v.swap(i, j);
            println!(
                "-swap/{:?} {:?} | {:?} | {:?}",
                &v[mid],
                &v[..left],
                &v[left..=right],
                &v[right + 1..]
            );
            i += 1;
            j -= 1;
            println!("loop(i = {}, j = {})", i, j);
        }
        v.swap(mid, j);
        println!(
            "eloop/{:?} {:?} | {:?} | {:?} i = {}, j = {}",
            &v[mid],
            &v[..left],
            &v[left..=right],
            &v[right + 1..],
            i,
            j
        );
        sort_range(v, left, i - 1);
        sort_range(v, j + 1, right);
    }
    sort_range(v, 0, v.len() - 1);
}

pub fn wrong_qsort<T: Ord>(v: &mut [T]) {
    fn sort_range<T: Ord>(v: &mut [T], left: usize, right: usize) {
        if left >= right {
            return;
        }
        let mid = (left + right) / 2;
        let (mut i, mut j) = (left, right);
        while i <= j {
            while v[i] < v[mid] {
                i += 1;
            }
            while v[mid] < v[j] {
                j -= 1;
            }
            if i < j {
                v.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        sort_range(v, left, i);
        sort_range(v, i + 1, right)
    }
    sort_range(v, 0, v.len() - 1);
}