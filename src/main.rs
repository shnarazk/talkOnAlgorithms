use toa::qsort::qsort;

fn main() {
    println!("Hello, world!");
    let mut vec: Vec<i32> = vec![-1, 4, 8, 1, 10, 8, -2, 9];
    let mut _vec: Vec<i32> = vec![5, 4, 3, 2, 1];
    let mut _vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    qsort(&mut vec);
}
