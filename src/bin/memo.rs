use toa::memo::*;

fn main() {
    println!("Hello, world!");
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
