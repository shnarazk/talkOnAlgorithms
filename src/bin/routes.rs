#![allow(dead_code)]
#![allow(unused_variables)]
use rand::prelude::SliceRandom;

fn number_of_simple_routes() -> usize {
    0
}

fn pathes_at(x: usize, y: usize) -> usize {
    0
}

const SIZE: usize = 17;

pub fn main() {
    let mut grid = [[1; SIZE]; SIZE];
    for j in 1..SIZE {
        for i in 1..SIZE {
            grid[j][i] = grid[j - 1][i] + grid[j][i - 1];
        }
    }
    dbg!(grid[SIZE - 1][SIZE - 1]);
    for i in 0..10 {
        println!("grid[{}][{}] = {}", i, i, grid[i][i]);
    }
    // make_map();
    set_routes();
}

pub fn make_map() {
    let mut seed = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 1..20 {
        let mut v: Vec<usize> = (1usize..20).collect();
        v.shuffle(&mut seed);
        vec.push(v);
    }
   
    for (i, v) in vec.iter().enumerate() {
        println!(r#"    \foreach \i [count=\j from 0] in {{{}}} \node[cell] at (\j, {}) {{$\i$}};"#,
                 v.iter().map(|i| format!("{}",i)).collect::<Vec<_>>().join(","),
                 i,
        )
    }
    for v in vec.iter() {
        println!("    vec.push(vec![{}]);",
                 v.iter().map(|i| format!("{}",i)).collect::<Vec<_>>().join(","),
        )
    }

}

fn set_routes () -> Vec<Vec<usize>> {
    let mut vec = Vec::new();
    vec.push(vec![2,7,5,9,16,1,14,4,15,17,8,11,12,19,3,18,10,13,6]);
    vec.push(vec![5,8,7,11,4,10,19,13,18,3,17,6,15,16,12,1,2,9,14]);
    vec.push(vec![10,3,18,5,1,7,14,17,12,2,11,19,15,6,13,16,4,9,8]);
    vec.push(vec![13,9,12,16,14,1,5,4,2,11,17,19,3,15,6,8,7,10,18]);
    vec.push(vec![7,19,5,14,3,8,12,18,6,10,17,4,1,13,9,16,15,2,11]);
    vec.push(vec![10,8,11,19,13,14,5,12,17,2,1,15,4,18,6,7,16,9,3]);
    vec.push(vec![13,19,15,2,3,18,11,10,17,8,16,4,14,7,9,5,6,1,12]);
    vec.push(vec![9,10,17,14,8,19,12,13,16,4,2,18,5,7,11,15,3,6,1]);
    vec.push(vec![3,15,5,1,19,4,14,9,11,2,13,16,18,7,12,8,10,17,6]);
    vec.push(vec![18,4,9,15,12,2,10,19,8,5,11,6,3,17,1,13,7,16,14]);
    vec.push(vec![16,3,9,6,19,5,4,12,1,13,8,17,2,11,14,15,7,18,10]);
    vec.push(vec![1,17,10,8,3,2,4,14,18,6,9,15,13,19,12,7,16,5,11]);
    vec.push(vec![5,6,3,10,7,13,1,15,16,17,8,2,9,11,18,14,19,4,12]);
    vec.push(vec![10,7,13,8,15,6,19,2,1,17,18,4,5,14,11,9,3,12,16]);
    vec.push(vec![17,15,5,19,3,6,2,10,13,11,9,8,7,14,1,18,16,12,4]);
    vec.push(vec![16,14,2,7,6,8,10,9,19,15,13,1,11,17,12,4,5,3,18]);
    vec.push(vec![10,7,17,4,11,14,3,12,1,13,18,19,8,9,16,5,6,2,15]);
    vec.push(vec![17,8,15,16,7,14,6,10,13,11,3,19,2,9,18,5,12,4,1]);
    vec.push(vec![11,2,4,14,15,13,19,12,18,16,10,7,1,3,6,8,17,9,5]);
    for j in 0..19 {
        for i in 0..19 {
            println!("{:>2},{:>2} = {:>2}", j, i, vec[j][i]);
        }
    }

    vec
}
