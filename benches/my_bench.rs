use bitflags::BitFlag;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn index_item_decode(c: &mut Criterion) {
    let mut bmap = BitFlag::<u128>::new();
    let len = BitFlag::<u128>::size() as u128;
    bmap.set(3, true);

    c.bench_function("bench set", |b| {
        let mut bf = BitFlag::<u128>::new();
        b.iter(|| {
            for i in 0..len {
                bf.set(black_box(i), true);
            }
        });
    });

    c.bench_function("bench set unchecked", |b| {
        let mut bf = BitFlag::<u128>::new();
        b.iter(|| {
            for i in 0..len {
                bf.set_unchecked(black_box(i), true);
            }
        });
    });

    c.bench_function("bench get", |b| {
        b.iter(|| {
            for i in 0..len {
                bmap.get(black_box(i));
            }
        });
    });

    c.bench_function("bench get unchecked", |b| {
        b.iter(|| {
            for i in 0..len {
                bmap.get_unchecked(black_box(i));
            }
        });
    });
}

criterion_group!(benches, index_item_decode);
criterion_main!(benches);
