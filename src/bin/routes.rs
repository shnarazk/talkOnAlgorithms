#![allow(dead_code)]
#![allow(unused_variables)]

const SIZE: usize = 17;

fn space_routes() -> [[usize; SIZE]; SIZE] {
    let mut grid = [[1; SIZE]; SIZE];
    for j in 1..SIZE {
        for i in 1..SIZE {
            grid[j][i] = grid[j - 1][i] + grid[j][i - 1];
        }
    }
    grid
}

fn ground_routes(horizon: usize) -> [[usize; SIZE]; SIZE] {
    let mut grid = [[1; SIZE]; SIZE];
    for j in horizon..SIZE {
        grid[0][j] = 0;
    }
    for j in 1..SIZE {
        for i in 1..SIZE {
            if i < j + horizon {
                grid[j][i] = grid[j - 1][i] + grid[j][i - 1];
            } else {
                grid[j][i] = 0;
            }
        }
    }
    for (i, v) in grid.iter().enumerate().take(5) {
        println!("grid[{}][{}] = {}", i, i, v[i]);
    }
    grid
}

pub fn main() {
    dbg!(space_routes()[15][15]);
    dbg!(ground_routes(5)[15][15]);
}
