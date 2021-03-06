use {
    criterion::{
        black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
        PlotConfiguration, Throughput,
    },
    rand::prelude::SliceRandom,
    toa::{bsort::bsort, qsort::qsort},
};

pub fn sort_benchmark(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("sort algorithms (log-log scaled)");
    group.plot_config(plot_config);
    group.sample_size(10);

    for size in [
        1000usize, 2000, 4000, 10_000, 20_000, 50_000, 100_000, 200_000,
    ] {
        let mut v: Vec<i32> = (0..size as i32).collect();

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("shuffle", size), &size, |b, _| {
            b.iter(|| black_box(v.shuffle(&mut rand::thread_rng())));
        });

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("bsort + shuffle", size), &size, |b, _| {
            b.iter(|| {
                v.shuffle(&mut rand::thread_rng());
                bsort(&mut v);
            })
        });

        group.bench_with_input(BenchmarkId::new("qsort + shuffle", size), &size, |b, _| {
            b.iter(|| {
                v.shuffle(&mut rand::thread_rng());
                qsort(&mut v);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
