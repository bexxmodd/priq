use priq::priq::PriorityQueue;

#[test]
fn pq_base() {
    let pq: PriorityQueue<f32, String> = PriorityQueue::new();
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
fn pq_put_four_and_grow() {
    let mut pq: PriorityQueue<u32, String> = PriorityQueue::new();
    pq.put(1, String::from("Erti"));
    pq.put(2, String::from("Ori"));
    pq.put(3, String::from("Sami"));
    pq.put(4, String::from("Otxi"));
    assert_eq!(4, pq.len());
}

#[test]
fn pq_peek_sorted_entry() {
    let mut pq: PriorityQueue<f32, String> = PriorityQueue::new();
    pq.put(1.1, String::from("Erti"));
    pq.put(2.2, String::from("Ori"));
    pq.put(3.3, String::from("Sami"));
    assert_eq!(1.1, pq.peek().unwrap().0);
    assert_eq!("Erti", pq.peek().unwrap().1);
}

#[test]
fn pq_peek_unsorted_entry() {
    let mut pq: PriorityQueue<f32, String> = PriorityQueue::new();
    pq.put(2.2, String::from("Ori"));
    pq.put(3.3, String::from("Sami"));
    pq.put(0.3, String::from("Me"));
    pq.put(1.1, String::from("Erti"));
    assert_eq!(0.3, pq.peek().unwrap().0);
    assert_eq!("Me", pq.peek().unwrap().1);
}

// #[test]
// fn priority_q_pop() {
//     let mut pq: PriorityQueue<f32, String> = PriorityQueue::new();
//     assert!(pq.pop().is_none());
// 
//     pq.push(1.0, String::from("Beka"));
//     pq.push(1.5, String::from("Nerse"));
//     pq.push(2.5, String::from("Ana"));
//     assert_eq!("Beka", pq.pop().unwrap().1);
//     assert!(pq.is_empty());
// }
// 
// #[test]
// fn priority_q_peek() {
//     let mut pq: PriorityQueue<f32, String> = PriorityQueue::new();
//     pq.put(1.0, String::from("Beka"));
//     pq.put(2.0, String::from("Ana"));
//     assert_eq!(2, pq.len());
//     assert_eq!("Beka", pq.peek().unwrap().1);
//     assert_eq!(2, pq.len());
// }
