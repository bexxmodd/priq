//! Benchmarks

#[macro_use]

extern crate bencher;
extern crate priq;

use priq::*;
use self::bencher::Bencher;

/// Benchmark putting 100 elements
fn pq_put_100(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    b.iter(|| {
        let n = 100_usize;
        (0..n).for_each(|i| { pq.put(i, i * 2); });
    });
}

/// Benchmark putting 1k elements
fn pq_put_1k(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    b.iter(|| {
        let n = 1_000_usize;
        (0..n).for_each(|i| { pq.put(i, i * 2); });
    });
}

/// Benchmark putting 10k elements
fn pq_put_10k(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    b.iter(|| {
        let n = 10_000_usize;
        (0..n).for_each(|i| { pq.put(i, i * 2); });
    });
}

/// Benchmark putting 100k elements
fn pq_put_100k(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    b.iter(|| {
        let n = 100_000_usize;
        (0..n).for_each(|i| { pq.put(i, i * 2); });
    });
}

/// Benchmark putting 1mil elements
fn pq_put_1mil(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    b.iter(|| {
        let n = 1_000_000_usize;
        (0..n).for_each(|i| { pq.put(i, i * 2); });
    });
}

/// Benchmark putting 100k elements with capacity constructor
fn pq_put_100k_wcap(b: &mut Bencher) {
    let n = 100_000_usize;
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::with_capacity(n);
    b.iter(|| {
        (0..n).for_each(|i| { pq.put(i, i * 2); });
    });
}

/// Benchmark putting 1mil elements
fn pq_put_1mil_wcap(b: &mut Bencher) {
    let n = 1_000_000_usize;
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::with_capacity(n);
    b.iter(|| {
        (0..n).for_each(|i| { pq.put(i, i * 2); });
    });
}

/// Benchmark pop-ing 100 elements
fn pq_pop_100(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    let n = 100_usize;
    (0..n).for_each(|i| { pq.put(i, i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { pq.pop(); });
    });
}

/// Benchmark pop-ing 1k elements
fn pq_pop_1k(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    let n = 1_000_usize;
    (0..n).for_each(|i| { pq.put(i, i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { pq.pop(); });
    });
}

/// Benchmark pop-ing 10k elements
fn pq_pop_10k(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    let n = 10_000_usize;
    (0..n).for_each(|i| { pq.put(i, i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { pq.pop(); });
    });
}

/// Benchmark pop-ing 100k elements
fn pq_pop_100k(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    let n = 100_000_usize;
    (0..n).for_each(|i| { pq.put(i, i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { pq.pop(); });
    });
}

/// Benchmark pop-ing 1mil elements
fn pq_pop_1mil(b: &mut Bencher) {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    let n = 1_000_000_usize;
    (0..n).for_each(|i| { pq.put(i, i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { pq.pop(); });
    });
}

benchmark_group!(
    benches,
    pq_put_100,
    pq_put_1k,
    pq_put_10k,
    pq_put_100k,
    pq_put_1mil,
    pq_pop_100,
    pq_pop_1k,
    pq_pop_10k,
    pq_pop_100k,
    pq_pop_1mil,
    pq_put_100k_wcap,
    pq_put_1mil_wcap,
);
benchmark_main!(benches);
