//! Benchmarks

#[macro_use]

extern crate bencher;
extern crate priq;

use priq::*;
use self::bencher::Bencher;
use std::collections::BinaryHeap;

/// Benchmark pushting 100 elements
fn bh_push_100(b: &mut Bencher) {
    let mut bh = BinaryHeap::new();
    let n = 100_usize;
    b.iter(|| {
        (0..n).for_each(|i| { bh.push(i * 2); });
    });
}

/// Benchmark pushting 1k elements
fn bh_push_1k(b: &mut Bencher) {
    let n = 1_000_usize;
    let mut bh = BinaryHeap::new();
    b.iter(|| {
        (0..n).for_each(|i| { bh.push(i * 2); });
    });
}

/// Benchmark pushting 10k elements
fn bh_push_10k(b: &mut Bencher) {
    let mut bh = BinaryHeap::new();
    let n = 10_000_usize;
    b.iter(|| {
        (0..n).for_each(|i| { bh.push(i * 2); });
    });
}

/// Benchmark pushting 100k elements
fn bh_push_100k(b: &mut Bencher) {
    let mut bh = BinaryHeap::new();
    let n = 1_000_000_usize;
    b.iter(|| {
        (0..n).for_each(|i| { bh.push(i * 2); });
    });
}

/// Benchmark pushting 1mil elements
fn bh_push_1mil(b: &mut Bencher) {
    let n = 1_000_000_usize;
    let mut bh = BinaryHeap::new();
    b.iter(|| {
        (0..n).for_each(|i| { bh.push(i * 2); });
    });
}

/// Benchmark pop-ing 100 elements
fn bh_pop_100(b: &mut Bencher) {
    let mut bh = BinaryHeap::new();
    let n = 100_usize;
    (0..n).for_each(|i| { bh.push(i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { bh.pop(); });
    });
}

/// Benchmark pop-ing 1k elements
fn bh_pop_1k(b: &mut Bencher) {
    let mut bh = BinaryHeap::new();
    let n = 1_000_usize;
    (0..n).for_each(|i| { bh.push(i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { bh.pop(); });
    });
}

/// Benchmark pop-ing 10k elements
fn bh_pop_10k(b: &mut Bencher) {
    let mut bh = BinaryHeap::new();
    let n = 10_000_usize;
    (0..n).for_each(|i| { bh.push(i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { bh.pop(); });
    });
}

/// Benchmark pop-ing 100k elements
fn bh_pop_100k(b: &mut Bencher) {
    let mut bh = BinaryHeap::new();
    let n = 100_000_usize;
    (0..n).for_each(|i| { bh.push(i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { bh.pop(); });
    });
}

/// Benchmark pop-ing 1mil elements
fn bh_pop_1mil(b: &mut Bencher) {
    let mut bh = BinaryHeap::new();
    let n = 1_000_000_usize;
    (0..n).for_each(|i| { bh.push(i * 2); });
    b.iter(|| {
        (0..n).for_each(|_| { bh.pop(); });
    });
}

benchmark_group!(
    benches,
    bh_push_100,
    bh_push_1k,
    bh_push_10k,
    bh_push_100k,
    bh_push_1mil,
    bh_pop_100,
    bh_pop_1k,
    bh_pop_10k,
    bh_pop_100k,
    bh_pop_1mil,
);
benchmark_main!(benches);
