use {
    toa::{
        lsearch::lsearch,
        bsearch::bsearch,
    },
    criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput},
    std::{collections::HashSet, iter},
};

pub fn search_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("search algorithms");
    for size in [
        100_000, 200_000, 400_000, 800_000, 1_600_000, 3_200_000, 6_400_000, 10_000_000, 20_000_000,
    ] {
        let mut set: HashSet<i32> = HashSet::new();
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            set.insert(i as i32);
        }
        for e in set.iter() {
            v.push(e);
        }
        // for bsearch
        let mut sorted_v = v.clone();
        sorted_v.sort();

        let mut key: i32 = 0;
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("lsearch", size), &size, |b, _| {
            b.iter(|| {
                key += 1;
                lsearch(&v, |&i| key == *i);
            })
        });

        let mut key: i32 = 0;
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("bsearch", size), &size, |b, _| {
            b.iter(|| {
                key += 1;
                bsearch(&sorted_v, |i| key.cmp(i));
            })
        });
    }
    group.finish();
}

criterion_group!(benches, search_benchmark);
criterion_main!(benches);
