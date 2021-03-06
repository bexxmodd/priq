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
#[should_panic]
fn pq_try_pop_when_empty() {
    let mut pq = PriorityQueue::<usize, usize>::new();
    pq.try_pop();
}

#[test]
fn pq_try_pop_base() {
    let mut pq = PriorityQueue::<usize, usize>::new();
    pq.put(4, 39);
    assert_eq!(39, pq.try_pop().1);
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
    assert!(!pq.is_empty());
    pq.clear();
    assert!(pq.is_empty());
}

#[test]
fn pq_drain() {
    let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
    assert!(!pq.is_empty());
    
    for (s, e) in pq.drain(..) { assert!(s > 0 && e > 0) };
    assert!(pq.is_empty());

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

#[test]
fn pq_into_sorted_vec() {
    let pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
    assert_eq!(3, pq.len());

    let mut res = pq.into_sorted_vec(); 
    assert_eq!(3, res.len());

    assert_eq!(55, res.pop().unwrap().1);
    assert_eq!(44, res.pop().unwrap().1);
    assert_eq!(11, res.pop().unwrap().1);
}

#[test]
fn pq_into_sorted_vec_with_nan() {
    let mut pq: PriorityQueue<f32, isize> = PriorityQueue::new();
    pq.put(1.1, 10);
    pq.put(f32::NAN, -1);
    pq.put(2.2, 20);
    pq.put(3.3, 30);
    pq.put(f32::NAN, -3);
    pq.put(4.4, 40);
    let res = pq.into_sorted_vec();
    
    assert_eq!(10, res[0].1);
    assert_eq!(20, res[1].1);
    assert_eq!(30, res[2].1);
    assert_eq!(40, res[3].1);
    assert!(res[4].1 < 0 && res[4].1 > -4);
    assert!(res[5].1 < 0 && res[5].1 > -4);
}

#[test]
fn pq_build_from_iter() {
    let iter = (0..5).into_iter()
                     .map(|i| (i, i * 2));
    let pq = PriorityQueue::from_iter(iter);
    assert_eq!(5, pq.len());
    assert_eq!(0, pq.peek().unwrap().1);
}

#[test]
fn pq_build_and_collect() {
    let pq: PriorityQueue<usize, usize> = (1..6).into_iter()
                                                .map(|i| (i, i + i))
                                                .collect();
    assert_eq!(5, pq.len());
    assert_eq!(1, pq.peek().unwrap().0);
}

#[test]
fn pq_into_ter() {
    let pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44), (2, 22)]);
    let res: PriorityQueue<u8, u8> = pq.into_iter()
                                       .filter(|(s, _)| s > &2)
                                       .collect();
    assert_eq!(2, res.len());
    assert_eq!(44, res.peek().unwrap().1);
}

#[test]
fn pq_drain_slice() {
    let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44), (2, 22)]);
    let res: PriorityQueue<usize, usize> = pq.drain(1..).collect();
    assert_eq!(3, res.len());
}

#[test]
fn pq_truncate_larger_len() {
    let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44), (2, 22)]);
    pq.truncate(6);
    assert_eq!(4, pq.len());
}

#[test]
fn pq_truncate() {
    let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44), (2, 22)]);
    pq.truncate(2);
    assert_eq!(2, pq.len());
    assert_eq!(1, pq.pop().unwrap().0);
    assert_eq!(2, pq.pop().unwrap().0);
    assert_eq!(None, pq.peek());
}

#[test]
fn pq_truncate_clear() {
    let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44), (2, 22)]);
    pq.truncate(0);
    assert!(pq.is_empty());
}

#[test]
fn pq_clone() {
    let pq1 = PriorityQueue::from([(5, 55), (1, 11), (4, 44), (2, 22)]);
    let pq2 = pq1.clone();
    assert_eq!(4, pq1.len());
    assert_eq!(4, pq2.len());
}

#[test]
fn pq_merge() {
    let mut pq1 = PriorityQueue::from([(5, 55), (1, 11), (3, 33), (2, 22)]);
    let mut pq2 = PriorityQueue::from([(4, 44), (6, 66)]);
    pq1.merge(&mut pq2);

    assert!(pq2.is_empty());
    assert_eq!(6, pq1.len());
    (1..=6).for_each(|i| { assert_eq!(i * 11, pq1.pop().unwrap().1); })
}

#[test]
fn pq_add_to_pq() {
    let pq1 = PriorityQueue::from([(5, 55), (1, 11), (4, 44), (2, 22)]);
    let pq2 = PriorityQueue::from([(8, 44), (1, 22)]);
    let res = pq1 + pq2;
    assert_eq!(6, res.len());
    assert_eq!(11, res.peek().unwrap().1);
}

#[test]
fn pq_high_number_of_nan() {
    let mut pq = PriorityQueue::new();
    pq.put(1f64, ());
    pq.put(3f64, ());
    pq.put(f64::NAN, ());
    pq.put(f64::NAN, ());
    println!("{:?}", pq.into_sorted_vec());
    // assert_eq!(1f64, pq.try_pop().0);
    // assert_eq!(3f64, pq.try_pop().0);
}

#[test]
fn pq_points_as_scores() {
    let mut pq = PriorityQueue::new();
    pq.put((1, 2), ());
    pq.put((0, 3), ());
    pq.put((2, 4), ());
    pq.put((5, 3), ());
    pq.put((6, 7), ());
    println!("{:?}", pq.into_sorted_vec());
}
