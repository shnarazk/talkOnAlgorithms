use {
    criterion::{
        black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
        PlotConfiguration, Throughput,
    },
    rand::prelude::SliceRandom,
    toa::{bsearch::bsearch, lsearch::lsearch},
};

pub fn search_benchmark(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("search algorithms");
    group.plot_config(plot_config);

    for size in [
        10_000usize,
        20_000,
        50_000,
        100_000,
        200_000,
        400_000,
        800_000,
        1_600_000,
    ] {
        let sorted: Vec<i32> = (0..size as i32).collect();
        let mut rand_v: Vec<i32> = sorted.clone();
        rand_v.shuffle(&mut rand::thread_rng());

        // lsearch
        let mut key: i32 = (size as i32) / 4;
        assert_eq!(rand_v.len(), size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("lsearch", size), &size, |b, _| {
            b.iter(|| {
                key += 1;
                black_box(lsearch(&rand_v, |i| key == *i));
            })
        });

        // bsearch
        assert_eq!(sorted.len(), size);
        let mut key: i32 = (size as i32) / 4;
        group.bench_with_input(BenchmarkId::new("bsearch", size), &size, |b, _| {
            b.iter(|| {
                key += 1;
                black_box(bsearch(&sorted, |i| key.cmp(i)));
            })
        });
    }
    group.finish();
}

criterion_group!(benches, search_benchmark);
criterion_main!(benches);
