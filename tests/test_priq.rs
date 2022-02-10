#![feature(test)]

use priq::PriorityQueue;

use std::cmp::Reverse;
use rand::{seq::SliceRandom, thread_rng};


#[test]
fn pq_base() {
    let pq: PriorityQueue<f32, String> = PriorityQueue::new();
    assert!(pq.is_empty());
}

#[test]
fn pq_wcapacity() {
    let pq: PriorityQueue<usize, usize> = PriorityQueue::with_capacity(100);
    assert!(pq.is_empty());
}

#[test]
fn pq_put_one() {
    let mut pq: PriorityQueue<usize, String> = PriorityQueue::new();
    pq.put(24, String::from("Erti"));
    assert_eq!(1, pq.len());
}

#[test]
fn pq_put_two() {
    let mut pq: PriorityQueue<f32, String> = PriorityQueue::new();
    pq.put(1.0, String::from("Erti"));
    pq.put(2.0, String::from("Ori"));
    assert_eq!(2, pq.len());
}

#[test]
fn pq_put_three() {
    let mut pq: PriorityQueue<f32, String> = PriorityQueue::new();
    pq.put(1.0, String::from("Erti"));
    pq.put(2.0, String::from("Ori"));
    pq.put(3.0, String::from("Sami"));
    assert_eq!(3, pq.len());
}

#[test]
fn pq_as_max_heap() {
    let mut pq: PriorityQueue<Reverse<u8>, String> = PriorityQueue::new();
    pq.put(Reverse(26), "Z".to_string());
    pq.put(Reverse(1), "A".to_string());
    assert_eq!(pq.pop().unwrap().1, "Z");
}

#[test]
fn pq_put_four_and_grow() {
    let mut pq: PriorityQueue<u8, String> = PriorityQueue::new();
    pq.put(1, String::from("Erti"));
    pq.put(2, String::from("Ori"));
    pq.put(3, String::from("Sami"));
    pq.put(4, String::from("Otxi"));
    assert_eq!(4, pq.len());
}

#[test]
fn pq_put_100000_items() {
    let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
    (0..100000).rev().for_each(|i| { pq.put(i, i * 2); });
    assert_eq!(100000, pq.len());
    assert_eq!(0, pq.peek().unwrap().1);
}

#[test]
fn pq_peek_base() {
    let mut pq: PriorityQueue<u8, String> = PriorityQueue::new();
    pq.put(1, String::from("Erti"));
    assert_eq!("Erti", pq.peek().unwrap().1);
}

#[test]
fn pq_peek_sorted_items() {
    let mut pq: PriorityQueue<u8, String> = PriorityQueue::new();
    pq.put(1, String::from("Erti"));
    pq.put(2, String::from("Ori"));
    pq.put(3, String::from("Sami"));
    assert_eq!(1, pq.peek().unwrap().0);
    assert_eq!("Erti", pq.peek().unwrap().1);
}

#[test]
fn pq_peek_unsorted_items() {
    let mut pq: PriorityQueue<u8, String> = PriorityQueue::new();
    pq.put(2, String::from("Ori"));
    pq.put(3, String::from("Sami"));
    pq.put(0, String::from("Me"));
    pq.put(1, String::from("Erti"));
    assert_eq!(0, pq.peek().unwrap().0);
    assert_eq!("Me", pq.peek().unwrap().1);
}

#[test]
fn pq_pop_base() {
    let mut pq: PriorityQueue<u8, String> = PriorityQueue::new();
    assert!(pq.pop().is_none());

    pq.put(1, String::from("Beka"));
    assert_eq!("Beka", pq.pop().unwrap().1);
    assert!(pq.is_empty());
}

#[test]
fn pq_pop_with_four_unordered_items() {
    let mut pq: PriorityQueue<u8, String> = PriorityQueue::new();
    pq.put(2, String::from("Ori"));
    pq.put(3, String::from("Sami"));
    pq.put(4, String::from("Otxi"));
    pq.put(0, String::from("Me"));
    assert_eq!(4, pq.len());
    assert_eq!("Me", pq.pop().unwrap().1);
    assert_eq!("Ori", pq.pop().unwrap().1);
    assert_eq!("Sami", pq.pop().unwrap().1);
    assert_eq!("Otxi", pq.pop().unwrap().1);
    assert!(pq.pop().is_none());
}

#[test]
fn pq_pop_100000_items_ordered() {
    let mut pq: PriorityQueue<i32, i32> = PriorityQueue::new();
    (0..100000).for_each(|i| {
        pq.put(i, -i);
    });
    assert_eq!(100000, pq.len());

    (0..100000).for_each(|i| {
        let item = pq.pop().unwrap();
        assert_eq!(i, item.0, "Incorrect order of Scores");
        assert_eq!(-i, item.1, "Incorrect write of Items");
    });
    assert!(pq.is_empty());
}

#[test]
fn pq_pop_100000_items_unordered() {
    let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    let mut rng = thread_rng();

    let mut scores: Vec<usize> = (0..100000).collect();
    scores.shuffle(&mut rng);

    (0..100000).for_each(|i| {
        pq.put(scores[i], i + 1);
    });
    assert_eq!(100000, pq.len());

    (0..100000).for_each(|i| {
        let item = pq.pop().unwrap();
        assert_eq!(i, item.0, "Incorrect order of Scores");
    });
    assert!(pq.is_empty());
}

#[test]
fn pq_from_vec() {
    let vec = vec![(5, 55), (1, 11), (4, 44), (2, 22), (3, 33)];
    let mut pq = PriorityQueue::from(vec);
    assert_eq!(5, pq.len());
    assert_eq!(11, pq.pop().unwrap().1);
    assert_eq!(22, pq.pop().unwrap().1);
}

#[test]
fn pq_from_slice() {
    let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
    assert_eq!(3, pq.len());
    assert_eq!(11, pq.pop().unwrap().1);
    assert_eq!(44, pq.pop().unwrap().1);
}

#[test]
fn pq_clear() {
    let mut pq: PriorityQueue<u8, String> = PriorityQueue::new();
    pq.put(1, String::from("Erti"));
    pq.put(2, String::from("Ori"));
    pq.put(3, String::from("Sami"));
    pq.put(4, String::from("Otxi"));
    pq.clear();
    assert!(pq.is_empty());
}

#[test]
fn pq_drain() {
    let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
    assert!(!pq.is_empty());
    
    for (s, e) in pq.drain() { assert!(s > 0 && e > 0) };
    assert!(pq.is_empty());

}

#[test]
fn pq_into_sorted_vec() {
    let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
    let mut res = pq.into_sorted_vec(); 
    assert_eq!(3, res.len());
    assert_eq!(55, res.pop().unwrap().1);
    assert_eq!(44, res.pop().unwrap().1);
    assert_eq!(11, res.pop().unwrap().1);
}

#[test]
fn pq_with_nan() {
    let mut pq: PriorityQueue<f32, isize> = PriorityQueue::new();
    pq.put(1.1, 10);
    pq.put(f32::NAN, -1);
    pq.put(2.2, 20);
    pq.put(3.3, 30);
    pq.put(f32::NAN, -3);
    pq.put(4.4, 40);
    
    (1..=4).for_each(|i| assert_eq!(i * 10, pq.pop().unwrap().1));
    assert!(0 > pq.pop().unwrap().1);
    assert!(0 > pq.pop().unwrap().1);
}

