#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::HashMap;

fn main() {
    for i in 3..=8 {
        dbg!(fib(i));
    }
    //let mut memo: [usize; 100] = [0; 100];
    //for i in 3..40 {
    //    dbg!(i, fib2(&mut memo, i));
    //}
}

// F(1) = 1
// F(2) = 1
// F(n) = F(n-1)+ F(n-2)
fn fib(n: usize) -> usize {
    if n < 2 { return 1; }
    let mut pre1 = 1;
    let mut pre2 = 1;
    let mut result = 1;
    for i in 3..=n {
        pre2 = pre1;
        pre1 = result;
        result = pre1 + pre2;
    }
    return result;
}

fn fib2(memo: &mut [usize;100], n: usize) -> usize {
    if memo[n] != 0 { return memo[n]; }
    if n < 3 { return 1; }
    let tmp = fib2(memo, n - 1) + fib2(memo, n - 2);
    memo[n] = tmp;
    return tmp;
}

/*
#[allow(dead_code)]
fn fib(memo: &mut HashMap<usize, usize>, n: usize) -> usize {
    {
        // check memo
        if let Some(x) = memo.get(&n) {
            return *x;
        }
    }
    // end of check memo
    if n < 2 { return 1; }
    let result = fib(memo, n - 1) + fib(memo, n - 2);
    {
        // insert the result to memo
        memo.insert(n, result);
    }
    result
}


fn fib2(n: usize) -> usize {
    if n < 2 { return 1; }
    let mut pre1 = 1;
    let mut pre2 = 1;
    let mut result = 1;
    for _ in 3..n {
        let tmp = result;
        result = pre1 + pre2;
        pre1 = tmp;
        pre2 = pre1;
    }
    result
}
*/
