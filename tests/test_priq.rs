use priq::priq::PriorityQueue;

#[test]
fn priority_q_base() {
    let pq: PriorityQueue<f32, String> = PriorityQueue::new();
    assert!(pq.is_empty());
}

#[test]
fn experiment() {
    let mut pq: PriorityQueue<f32, String> = PriorityQueue::new();
    pq.put(7.4, String::from("Air Bud"));
    pq.put(8.3, String::from("Luke"));
    pq.put(1.1, String::from("Top Gun"));
    assert_eq!("Top Gun", pq.pop().unwrap().1);
    assert_eq!("Air Bud", pq.pop().unwrap().1);
}

// #[test]
// fn priority_q_put() {
//     let mut pq: PriorityQueue<f32, String> = PriorityQueue::new();
//     pq.put(0, 2.0, String::from("Beka"));
//     pq.put(1, 3.0, String::from("Ana"));
//     println!("Length: {}", pq.len());
//     let (score, value) = pq.pop().unwrap();
//     println!("first {}", value);
//     // let second = pq.pop().unwrap().1;
// 
//     pq.put(0, 1.0, String::from("Nerse"));
//     assert_eq!("Nerse", pq.peek().unwrap().1);
// }

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
