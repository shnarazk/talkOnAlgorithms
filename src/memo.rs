use {
    lazy_static::lazy_static,
    std::{collections::HashMap, sync::RwLock},
};

lazy_static! {
    static ref MEMO: RwLock<HashMap<usize, usize>> = RwLock::new(HashMap::new());
}

/// `n`のフィボナッチ数を返す。
/// メモ化していることを利用ユーザに公開する必要があるかは？
/// ```
/// use crate::toa::memo::*;
/// assert_eq!(fib(1), slow_fib(1));
/// assert_eq!(fib(2), slow_fib(2));
/// assert_eq!(fib(3), slow_fib(3));
/// assert_eq!(fib(20), slow_fib(20));
/// assert!((1..20).all(|n| fib(n) == slow_fib(n)));
/// ```
pub fn fib(n: usize) -> usize {
    if let Ok(hash) = MEMO.read() {
        if let Some(r) = hash.get(&n) {
            return *r;
        }
    }
    let n_1 = if n <= 2 { 1 } else { fib(n - 1) };
    let n_2 = if n <= 2 { 0 } else { fib(n - 2) };
    let result = n_1 + n_2;
    if let Ok(mut hash) = MEMO.write() {
        hash.insert(n, result);
    }
    result
}

/// フィボナッチ数を返す。遅いので使わないこと。
pub fn slow_fib(n: usize) -> usize {
    let n_1 = if n <= 2 { 1 } else { slow_fib(n - 1) };
    let n_2 = if n <= 2 { 0 } else { slow_fib(n - 2) };
    n_1 + n_2
}
