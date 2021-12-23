#![allow(dead_code)]
#![allow(clippy::needless_range_loop)]
pub fn main() {
    dbg!(pascal(15, 15));
    let mut memo: [[usize; 16]; 16] = [[0; 16]; 16];
    dbg!(pascal2(&mut memo, 15, 15));
    // draw_pascal();
    // count_pascal();
    
}

fn pascal(j: usize, i :usize) -> usize {
    if j == 0 || i == 0 { return 1; }
    return pascal(j - 1, i) + pascal(j, i - 1);
}

fn pascal2(memo: &mut [[usize; 16]; 16], j: usize, i :usize) -> usize {
    // (10, 8) (8, 10) => (10 min 8, 10 max 8)
    let jj = j.min(i);
    let ii = j.max(i);
    if memo[jj][ii] != 0 { return memo[jj][ii]; }
    if j == 0 || i == 0 { return 1; }
    let tmp = pascal2(memo, j - 1, i) + pascal2(memo, j, i - 1);
    memo[jj][ii] = tmp;
    return tmp;
}


/// 表表示用1
fn draw_pascal() {
    for j in 0..8 {
        for i in 0..8 {
            print!("{:>6}, ", pascal(j, i));
        }
        println!()
    }
}

/// 表表示用2
fn count_pascal() {
    const SIZE: usize = 11;
    let mut count: [[usize; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    fn pascal(count: &mut [[usize; SIZE]; SIZE], j: usize, i :usize) -> usize {
        count[j][i] += 1;
        if j == 0 || i == 0 { return 1; }
        pascal(count, j - 1, i) + pascal(count, j, i - 1)
    }
    dbg!(pascal(&mut count, 7, 7));
    for j in 0..8 {
        for i in 0..8 {
            print!("{:>5}, ", count[j][i]);
        }
        println!()
    }
}
