use std::cmp::Ordering;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn consecutive_sum_for(num: usize) -> bool {
    let range: Vec<usize> = (1..=num).collect();

    for n in &range {
        for w in range.windows(*n) {
            if w.iter().sum::<usize>() == num {
                return true;
            }
        }
    }

    false
}

fn consecutive_sum_iter(num: usize) -> bool {
    let range: Vec<usize> = (1..=num).collect();
    range
        .windows(2)
        .chain((1..=num / 4).flat_map(|n| range.windows(n + 2)))
        .any(|w| w.iter().sum::<usize>() == num)
}

fn consecutive_sum_iter_better(num: usize) -> bool {
    let range: Vec<usize> = (1..=num).collect();

    let mut res = false;

    for n in &range {
        if !res {
            range
                .windows(*n + 1)
                .for_each(|w| res = w.iter().sum::<usize>() == num);
            break;
        }
    }
    res
}

fn consecutive_sum_weird(num: usize) -> bool {
    let mut start = 1;
    let mut end = 1;
    let mut sum = 1;

    while start <= num / 2 {
        match sum.cmp(&num) {
            Ordering::Less => {
                end += 1;
                sum += end;
            }
            Ordering::Greater => {
                sum -= start;
                start += 1;
            }
            Ordering::Equal => return true,
        }
    }

    false
}

const DATA: usize = 20;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("consecutive iter", |b| {
        b.iter(|| consecutive_sum_iter(black_box(DATA)))
    });
    c.bench_function("consecutive for", |b| {
        b.iter(|| consecutive_sum_for(black_box(DATA)))
    });
    c.bench_function("consecutive iter better", |b| {
        b.iter(|| consecutive_sum_iter_better(black_box(DATA)))
    });

    c.bench_function("consecutive weird", |b| {
        b.iter(|| consecutive_sum_weird(black_box(DATA)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
