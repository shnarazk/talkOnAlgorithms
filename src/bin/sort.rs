use toa::qsort::qsort;

fn main() {
    println!("Hello, world!");
    let mut vec: Vec<i32> = vec![1, 2, 3, 1, 2, 3, 4, 5, 1, 2, 3];
    qsort(&mut vec);
    println!("{:?}", &vec);
}
