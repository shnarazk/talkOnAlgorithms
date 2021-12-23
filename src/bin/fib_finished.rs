#![allow(dead_code)]
use std::collections::HashMap;

// memo[1] = 1
// memo[2] = 1
// memo[3] = 2
// memo[100] = 0
fn main() {
    for i in 2..100_000_000 {
        if is_prime(i) {
            println!("{:>10}", i);
        }
    } 
    // let mut memo: HashMap<usize, usize> = HashMap::new();
    // for n in 1 ..33 {
    //     println!("fib({:>2}) = {:>10}", n, fib(&mut memo, n));
    // }
    // for n in 1 ..33 {
    //     println!("fib({:>2}) = {:>10}", n, fib2(n));
    // }

    // let mut memo2: HashMap<(usize, usize), usize> = HashMap::new();
    // let size: usize = 16;
    // for j in 0..size {
    //     for i in 0..size {
    //         print!("{:>10}", pascal(&mut memo2, j, i));
    //     }
    //     println!();
    // }
}

fn is_prime(n: usize) -> bool {
    for i in 2..=(n as f64).sqrt().floor() as usize {
    // for i in 2..n {
        if n % i == 0 {
            return false;
        } 
    }
    true
}

fn pascal(memo: &mut HashMap<(usize, usize), usize>, j: usize, i: usize) -> usize {
    {
        // check memo
        let jj = j.min(i);
        let ii = j.max(i);
        if let Some(x) = memo.get(&(jj, ii)) {
            return *x;
        }
    }
    if j == 0 || i == 0 { return 1; }
    let result = pascal(memo, j - 1, i) + pascal(memo, j, i - 1);
    {
        // insert the result to memo
        let jj = j.min(i);
        let ii = j.max(i);
        memo.insert((jj, ii), result);
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
