use toa::lsearch::lsearch;
use toa::qsort::qsort;
use {std::collections::HashSet, toa::memo::*};

fn main() {
    println!("Hello, world!");

    let mut set: HashSet<usize> = HashSet::new();
    for i in 0..40 {
        set.insert(i);
    }
    for x in set.iter() {
        println!("{:?}", x);
    }

    {
        let size = 20_000_000;
        let mut set: HashSet<i32> = HashSet::new();
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            set.insert(i as i32);
        }
        for e in set.iter() {
            v.push(e);
        }
        println!("start lsearch on a vector: {:?} (N={})", &v[..10], v.len());
        dbg!(lsearch(&v, |e| **e == 10000i32));
    }

    // let mut vec: Vec<i32> = vec![1, 2, 3, 1, 2, 3, 4, 5, 1, 2, 3];
    // qsort(&mut vec);
    // println!("{:?}", &vec);
    let n = 46;
    // main関数から1回しか呼ばなくても再帰しているのでメモ化は有効
    println!("     fib({:>2}) = {:>11}", n, fib(n));
    println!("slow_fib({:>2}) = {:>11}", n, slow_fib(n));
    // メモに残っているので2回目以降はO(1)
    for _i in 30..=48 {
        println!("     fib({:>2}) = {:>11}", n, fib(n));
        println!("slow_fib({:>2}) = {:>11}", n, slow_fib(n));
    }
    // メモに残っているので新規分だけ計算
    for i in 30..=48 {
        println!("     fib({:>2}) = {:>11}", i, fib(i));
        println!("slow_fib({:>2}) = {:>11}", i, slow_fib(i));
    }
}
