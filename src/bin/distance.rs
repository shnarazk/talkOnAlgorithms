#![allow(dead_code)]
#![allow(unused_variables)]
use toa::dijkstra;
use rand::prelude::*;
use std::collections::HashMap;

fn min_dist(grid: &[Vec<usize>], j: usize, i: usize) -> usize {
    if j == 0 && i == 0 {
        return 1;
    }
    let mut npathes = usize::MAX;
    if 0 < j {
        npathes = npathes.min(grid[j - 1][i] + min_dist(grid, j - 1, i));
    }
    if 0 < i {
        npathes = npathes.min(grid[j][i - 1] + min_dist(grid, j, i - 1));
    }
    npathes
}

fn memoized_min_dist(memo: &mut HashMap<(usize, usize), usize>, grid: &[Vec<usize>], j: usize, i: usize) -> usize {
    if let Some(v) = memo.get(&(j, i)) {
        return *v;
    }
    let mut npathes = usize::MAX;
    if 0 < j {
        npathes = npathes.min(grid[j - 1][i] + memoized_min_dist(memo, grid, j - 1, i));
    }
    if 0 < i {
        npathes = npathes.min(grid[j][i - 1] + memoized_min_dist(memo, grid, j, i - 1));
    }
    memo.insert((j, i), npathes);
    npathes
}

const SIZE: usize = 17;

pub fn main() {
    // make_map();
    let map = set_routes();
    // 問題1.2
    // dbg!(min_dist(&map, 16, 16));
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    memo.insert((0, 0), 0);
    dbg!(memoized_min_dist(&mut memo, &map, 16, 16));
    for j in (0..SIZE).rev() {
        for i in 0..SIZE {
            if let Some(x) = memo.get(&(j, i)) {
                print!("{:>4}", x);
            } else {
                print!("   ");
            }
        }
        println!();
    }
    // 問題1.2.2
    let dist = dijkstra::build_up(&map);
    dbg!(dist[16][16]);
}

pub fn make_map() {
    let seed = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..SIZE {
        let mut v: Vec<usize> = Vec::new();
        for _ in 0..SIZE {
            v.push(((random::<usize>() % 10) + (random::<usize>() % 10)) % 9 + 1);
        }
        assert_eq!(v.len(), SIZE);
        vec.push(v);
    }
    assert_eq!(vec.len(), SIZE);
    for (i, v) in vec.iter().enumerate() {
        println!(r#"    \foreach \i [count=\j from 0] in {{{}}} \node[cell] at (\j, {}) {{$\i$}};"#,
                 v.iter().map(|i| format!("{}",i)).collect::<Vec<_>>().join(","),
                 i,
        )
    }
    for v in vec.iter() {
        println!("        vec![{}],",
                 v.iter().map(|i| format!("{}",i)).collect::<Vec<_>>().join(","),
        )
    }
    for (j,v) in vec.iter().enumerate() {
        for (i, x) in v.iter().enumerate() {
            println!("{:>2},{:>2} = {:>2}", j, i, x);
        }
    }
}

fn set_routes () -> Vec<Vec<usize>> {
    let vec = vec![
        vec![9,9,4,8,7,3,3,4,7,8,4,2,6,1,3,1,6],
        vec![2,2,4,8,4,9,9,5,4,3,5,5,7,5,1,7,5],
        vec![5,6,1,8,6,5,4,3,6,9,7,1,9,3,9,4,4],
        vec![8,2,1,8,3,5,5,6,7,8,1,2,7,5,8,5,4],
        vec![3,1,5,7,9,1,6,6,1,8,6,3,3,8,3,9,8],
        vec![6,2,9,4,4,9,6,7,6,3,7,2,7,1,7,4,2],
        vec![7,3,5,1,9,6,2,3,2,4,7,4,9,7,3,6,2],
        vec![7,2,9,3,5,7,9,5,9,6,4,8,8,5,1,8,1],
        vec![8,9,7,8,2,7,8,3,8,8,6,2,3,8,4,8,3],
        vec![2,1,2,3,6,7,9,2,5,8,5,5,5,6,3,6,4],
        vec![2,6,4,7,6,8,2,5,7,4,4,9,8,9,3,6,8],
        vec![1,7,8,5,3,7,1,7,3,1,9,7,2,6,9,6,4],
        vec![8,6,9,2,6,9,7,8,5,2,6,5,9,9,5,2,4],
        vec![6,9,1,5,2,6,5,8,8,4,9,9,2,2,7,3,9],
        vec![6,3,5,8,7,2,6,1,6,7,8,6,4,9,8,6,8],
        vec![5,3,2,8,5,2,4,6,2,6,1,8,9,5,6,3,2],
        vec![7,8,7,2,4,2,1,2,3,2,5,3,2,7,3,9,6],
    ];
    vec
}
