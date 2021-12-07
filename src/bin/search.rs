use {std::collections::HashSet, toa::lsearch::lsearch};

fn main() {
    println!("Hello, world!");
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
