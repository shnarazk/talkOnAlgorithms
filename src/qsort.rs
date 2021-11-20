use std::fmt::Debug;

///
///```
/// use crate::toa::qsort::*;
///
/// check_implementation(qsort, vec![-1, 4, 8, 1, 10, 8, -2, 9]);
/// check_implementation(qsort, vec![1, 1]);
/// check_implementation(qsort, vec![1]);
/// check_implementation(qsort, vec![1, 2, 3, 4, 5]);
/// check_implementation(qsort, vec![5, 4, 3, 2, 1]);
/// check_implementation(qsort, vec![1 ,2, 3, 1, 2, 3, 4, 5, 1, 2, 3]);
///```
pub fn qsort<T: Ord>(v: &mut [T]) {
    sort_on(v, 0, v.len());
}

fn partition<T: Ord>(v: &mut [T], beg: usize, end: usize) -> usize {
    let mut i = beg;
    for j in beg..end {
        if v[j] <= v[end - 1] {
            v.swap(i, j);
            i += 1;
        }
    }
    i.min(end - 1)
}

fn sort_on<T: Ord>(v: &mut [T], beg: usize, end: usize) {
    if beg + 1 >= end {
        return;
    }
    let p = partition(v, beg, end); // assert!(0 < end);
    sort_on(v, beg, p);
    sort_on(v, p, end);
}

pub fn check_implementation<T: Copy + Ord + Debug, V: AsRef<[T]>>(f: impl Fn(&mut [T]), v: V) {
    let mut v1 = v.as_ref().iter().copied().collect::<Vec<T>>();
    let mut v2 = v.as_ref().iter().copied().collect::<Vec<T>>();
    f(&mut v1);
    v2.sort();
    assert_eq!(v1, v2);
}

fn _sort_on<T: Ord + Debug>(v: &mut [T], left: usize, right: usize) {
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
                return _sort_on(v, left + 1, right);
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
                return _sort_on(v, left, right - 1);
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
    _sort_on(v, left, i - 1);
    _sort_on(v, j + 1, right);
}

#[allow(dead_code)]
fn wrong_qsort<T: Ord>(v: &mut [T]) {
    fn sort_on<T: Ord>(v: &mut [T], left: usize, right: usize) {
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
        sort_on(v, left, i);
        sort_on(v, i + 1, right)
    }
    sort_on(v, 0, v.len() - 1);
}
