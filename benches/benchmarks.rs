use cavalier_contours::polyline::PlineSource;
use cavalier_contours::polyline::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod test_polylines;
use test_polylines::*;

fn repeat_offsets(polyline: &Polyline<f64>, offset: f64, count: usize) {
    for i in 1..=count {
        let offset = i as f64 * offset;
        PlineSource::parallel_offset(*black_box(&polyline), black_box(offset));
        PlineSource::parallel_offset(*black_box(&polyline), black_box(-offset));
    }
}

fn polyline_offset_benchmarks(c: &mut Criterion) {
    let profile1 = profile1();
    c.bench_function("profile1", |b| {
        b.iter(|| repeat_offsets(&profile1, 0.1, 40))
    });

    let profile2 = profile2();
    c.bench_function("profile2", |b| {
        b.iter(|| repeat_offsets(&profile2, 0.1, 40))
    });

    let profile1_no_arcs = profile1_no_arcs();
    c.bench_function("profile1_no_arcs", |b| {
        b.iter(|| repeat_offsets(&profile1_no_arcs, 0.1, 40))
    });

    let profile2_no_arcs = profile2_no_arcs();
    c.bench_function("profile2_no_arcs", |b| {
        b.iter(|| repeat_offsets(&profile2_no_arcs, 0.1, 40))
    });

    let pathological1 = pathological1(100);
    c.bench_function("pathological1", |b| {
        b.iter(|| repeat_offsets(&pathological1, 1.0, 30))
    });

    let pathological1_no_arcs = pathological1_no_arcs(100);
    c.bench_function("pathological1_no_arcs", |b| {
        b.iter(|| repeat_offsets(&pathological1_no_arcs, 1.0, 30))
    });
}

criterion_group!(benches, polyline_offset_benchmarks,);
criterion_main!(benches);
