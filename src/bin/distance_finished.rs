#![allow(dead_code)]
#![allow(unused_variables)]
use rand::prelude::*;

fn number_of_simple_routes() -> usize {
    0
}

fn pathes_at(x: usize, y: usize) -> usize {
    0
}

const SIZE: usize = 17;

pub fn main() {
    // make_map();

    let mut map = set_routes();
    let mut dist = map.clone();

    for j in (0..SIZE).rev() {
        for i in 0..SIZE {
            print!("{:>2} ", map[j][i]);
        }
        println!();
    }
    // problem 1.2
    dist[0][0] = 0;
    for j in 0..SIZE {
        for i in 0..SIZE {
            match (j, i) {
                (0, 0) => (),
                (_, 0) => dist[j][i] = map[j - 1][i] + dist[j - 1][i],
                (0, _) => dist[j][i] = map[j][i - 1] + dist[j][i - 1],
                _ => dist[j][i] = (map[j - 1][i] + dist[j - 1][i]).min(map[j][i - 1] + dist[j][i - 1]),
            }
        }
    }
    for j in (0..SIZE).rev() {
        for i in 0..SIZE {
            print!("{:>4} ", dist[j][i]);
        }
        println!();
    }

    // problem 1.3
    let mut route: Vec<(usize, usize)> = Vec::new();
    let mut loc: (usize, usize) = (16, 16);
    while loc != (0, 0) {
        match loc {
            (0, i) => loc.1 -= 1,
            (j, 0) => loc.0 -= 1,
            (j, i) => {
                if dist[j - 1][i] < dist[j][i - 1] {
                    loc.0 -= 1;
                } else {
                    loc.1 -= 1;
                }
            }
        }
        route.push(loc);
    }
    print!("A");
    for l in route.iter().rev() {
        print!(" ({},{}) ->", l.0, l.1);
    }
    println!("B(16, 16)");

    // for LaTeX

    for (j, v) in dist.iter().enumerate() {
        println!("\\foreach \\i [count=\\j from 0] in {{{}}} \\node[cell] at (\\j, {}) {{$\\i$}};",
                 (0..SIZE).map(|i| format!("{}", v[i])).collect::<Vec<_>>().join(","),
                 j);
    }

    print!("{{ ");
    for l in route.iter().rev() {
        print!("{}/{},", l.0, l.1);
    }
    println!("16/16 }}");
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
    vec[SIZE-1][SIZE-1] = 0;
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
        vec![7,8,7,2,4,2,1,2,3,2,5,3,2,7,3,9,0],
    ];
    vec
}
