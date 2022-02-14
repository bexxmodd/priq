#![feature(slice_range)]
//! Priority queue (min/max heap) using raw binary heap.
//!
//! `PriorityQueue` is built using raw array for efficient performance.
//!
//! There are two major reasons what makes this `PriorityQueue` different from
//! other binary heap implementations currently available:
//!
//! 1 - Allows data ordering to scores with `PartialOrd`.
//!     * Every other min-max heap requires [total ordering](https://bit.ly/3GCWvYL) 
//!     of scores (e.g. should implement `Ord` trait). This can be an issue, 
//!     for example, when you want to order items based on a float scores, 
//!     which doesn't implement `Ord` trait.
//!     * Because of partial ordering, non-comparable values are thrown in 
//!     the end of the queue. One will see non-comparable values only after all 
//!     the comparable elements have been `pop`-ed.
//!     * You can read about Rust's implementation or `Ord`, `PartialOrd` and 
//!     what's the different [here](https://bit.ly/3J7NwQI)
//!
//! 2 - Separation of score and item you wish to store.
//!     * This frees enforcement for associated items to implement any ordering.
//!     * Makes easier to evaluate items' order.
//!
//! 3 - Equal scoring items are stored at first available free space.
//!     * This gives performance boost for large number of entries.
//!
//! 4 - Easy to use!
//!
//! You can read more about this crate on [my blog](https://www.bexxmodd.com)

extern crate rand;

use std::cmp::Ordering;
use std::mem;
use std::ops::Add;
use std::ops::Range;
use std::ops::RangeBounds;
use std::ptr;
use std::cmp;
use std::marker;
use std::ops::{Deref, DerefMut};
use std::convert::From;
use std::slice;

mod rawpq;
use rawpq::RawPQ;

/// A Min-Max Heap with designated arguments for `score` and associated `item`!
///
/// A `Default` implementation is a Min-Heap where the top node (root) is the 
/// lowest scoring element:
///
/// ```text
///               10
///             /    \
///           58      70
///          /  \    /  \
///         80  92  97  99
///
/// ```
///
/// > The value of Parent Node is small than Child Node.
///
/// Every parent node, including the top (root) node, is less than or equal to 
/// equal to the right child.
///
/// `PriorityQueue ` allows duplicate score/item values. When you [`put`]the 
/// item with a similar score that’s already in the queue new entry will be 
/// stored at the first empty location in memory. This gives an incremental 
/// performance boost (instead of resolving by using the associated item as a 
/// secondary tool to priority evaluation). Also, this form of implementation 
/// doesn’t enforce for the element `T` to have any implemented ordering. This
/// guarantees that the top node will always be of minimum value.
///
/// You can initialize an empty `PriorityQueue` and later add items:
///
/// ```
/// use priq::PriorityQueue;
///
/// // create queue with `usize` key and `String` elements
/// let pq: PriorityQueue<usize, String> = PriorityQueue::new();
/// ```
///
/// You can _heapify_ a `Vec` and/or a `slice`:
///
/// ```
/// use priq::PriorityQueue;
///
/// let pq_from_vec = PriorityQueue::from(vec![(5, 55), (1, 11), (4, 44)]);
/// let pq_from_slice = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
/// ```
///
/// You can build priority queue from any iterable and collect elements.
///
/// ```
/// use priq::PriorityQueue;
/// 
/// // this will yield: (1, 2), (2, 4), (3, 6), (4, 8)
/// let pq: PriorityQueue<_, _> = (1..5).into_iter()
///                                     .map(|i| (i, i + i))
///                                     .collect();
/// assert_eq!(4, pq.len());
/// assert_eq!(1, pq.peek().unwrap().0);
/// ```
///
/// # Partial Ordering
///
/// Because `priq` allows `score` arguments that only implement `PartialOrd`, 
/// elements that can't be compared are evaluated and are put in the back of
/// the queue:
///
/// ```
/// use priq::PriorityQueue;
///
/// let mut pq: PriorityQueue<f32, isize> = PriorityQueue::new();
///
/// pq.put(1.1, 10);
/// pq.put(f32::NAN, -1);
/// pq.put(2.2, 20);
/// pq.put(3.3, 30);
/// pq.put(f32::NAN, -3);
/// pq.put(4.4, 40);
/// 
/// (1..=4).for_each(|i| assert_eq!(i * 10, pq.pop().unwrap().1));
///
/// // NAN scores will not have deterministic order
/// // they are just stored after all the comparable scores
/// assert!(0 > pq.pop().unwrap().1);
/// assert!(0 > pq.pop().unwrap().1);
/// ```
/// 
/// # Time
/// 
/// The standard usage of this data structure is to [`put`] an element to the 
/// queue and [`pop`] to remove the top element and peek to check what’s the 
/// top element in the queue. The stored structure of the elements is a balanced
/// tree realized using an array with a contiguous memory location. This allows
/// maintaining a proper parent-child relationship between put-ed items.
///
/// [`put`]: PriorityQueue::put
/// [`peek`]: PriorityQueue::peek
/// [`pop`]: PriorityQueue::pop
///
///
/// Runtime complexity with Big-O Notation:
/// 
/// | method    | Time Complexity |
/// |-----------|-----------------|
/// | [`put`]   | _O(log(n))_     |
/// | [`pop`]   | _O(log(n))_     |
/// | [`peek`]  | _O(1)_          |
///
/// You can also iterate over elements using for loop but the returned slice 
/// will not be properly order as the heap is re-balanced after each insertion 
/// and deletion. If you want to grab items in a proper priority call [`pop`] 
/// in a loop until it returns `None`.
///
///
/// # Custom `struct`
///
/// What if you want to custom `struct ` without having a separate and 
/// specific score? You can pass the `struct`’s clone as a `score` and as an 
/// associated value, but if in this kind of scenario I’d recommend using
/// [`BinaryHeap`] as it better fits the purpose.
///
///
/// # Min-Heap
///
/// If instead of Min-Heap you want to have Max-Heap, where the highest-scoring 
/// element is on top you can pass score using [`Reverse`] or a custom [`Ord`] 
/// implementation can be used to have custom prioritization logic.
///
/// [`BinaryHeap`]: std::collections::BinaryHeap
/// [`Reverse`]: std::cmp::Reverse
///
/// # Example
///
/// ```
/// use priq::PriorityQueue;
/// use std::cmp::Reverse;
///
/// let mut pq: PriorityQueue<Reverse<u8>, String> = PriorityQueue::new();
///
/// pq.put(Reverse(26), "Z".to_string());
/// pq.put(Reverse(1), "A".to_string());
///
/// assert_eq!(pq.pop().unwrap().1, "Z");
/// ```
#[derive(Debug)]
pub struct PriorityQueue<S, T> 
where
    S: PartialOrd,
{
    data: RawPQ<S, T>,
    len: usize,
}


impl<S, T> PriorityQueue<S, T>
where
    S: PartialOrd,
{
    /// Create an empty `PriorityQueue`
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let pq: PriorityQueue<f32, String> = PriorityQueue::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        PriorityQueue {
            data: RawPQ::new(),
            len: 0,
        }
    }

    /// If you expect that you’ll be putting at least `n` number of items in 
    /// `PriorityQueue` you can create it with space of at least elements equal 
    /// to `cap`. This can boost the performance for a large number of sets 
    /// because it'll eliminate the need to grow the underlying array often.
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let pq: PriorityQueue<usize, usize> = PriorityQueue::with_capacity(100);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        PriorityQueue {
            data: RawPQ::with_capacity(cap),
            len: 0,
        }
    }

    /// Inserts an element in the heap.
    ///
    /// # Examples
    ///
    /// ```
    ///use priq::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<usize, String> = PriorityQueue::new();
    /// pq.put(1, "Velkhana".to_string());
    /// pq.put(2, "Shara".to_string());
    /// assert_eq!(2, pq.len());
    /// assert_eq!("Velkhana", pq.pop().unwrap().1);
    /// ```
    ///
    /// Element’s exact location will be determined based on its `score`. The 
    /// element will start as a last element in the `PriorityQueue` and then 
    /// percolate up using insertion sort operations on the path from the end 
    /// to the root to find the correct place for it.
    ///
    /// For example, we have a tree with scores **[2, 3, 4, 6, 9, 5, 4]** and 
    /// we want to `put` an element with a score of ***1***:
    ///
    /// ```text
    ///      2                          2                          X <- 1              
    ///    /   \                      /   \                      /   \
    ///   3     4                    3     X <- 1               3     2
    ///  / \   / \                  / \   / \                  / \   / \
    /// 6   9 5   X <- 1           6   9 5   4                6   9 5   4
    ///
    ///    step 1.                    step 2.                    step 3.
    /// ```
    ///
    /// On a `PriorityQueue` with `len == 7` to `put` a new element it made 
    /// three operations, from the last position to the top (worst case).
    /// 
    /// # Time Complexity
    ///
    /// For worst case scenario ***O(log(n))***.
    ///
    pub fn put(&mut self, score: S, item: T) {
        if self.cap() == self.len { self.data.grow(); }
        self.len += 1;

        // SAFETY: We're writing new element in the back of the array. We ensure
        //      that this is safe by first checking if we have enough capacity 
        //      allocated, and if not we grow it to accommodate new entries.
        unsafe {
            ptr::write(self.ptr().add(self.len - 1), (score, item))
        };
        self.heapify_up(self.len - 1);
    }

    /// Get the top priority element from `PriorityQueue`.
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<u8, String> = PriorityQueue::new();
    /// pq.put(2, String::from("Odo"));
    /// pq.put(3, String::from("Vaal"));
    /// pq.put(0, String::from("Nergi"));
    /// assert_eq!("Nergi", pq.pop().unwrap().1);
    /// assert_eq!("Odo", pq.pop().unwrap().1);
    /// ```
    ///
    /// Element will be removed from the `PriorityQueue` and next lowest 
    /// scoring item will be promoted as a top element (highest scoring if 
    /// `PriorityQueue` is used as a Max Heap).
    ///
    /// After priority is removed and returned `PriorityQueue` will balance 
    /// itself by promoting the next lowest scoring (or highest if Max Heap) 
    /// element as a top node. First the last element in the array is moved as 
    /// a top and percolated down with an insertion sort algorithm to find its
    /// correct place. This allows the next prioritized item to end at top.
    ///
    /// For example, we have a tree with scores **[1, 3, 2, 6, 9, 5, 4]**. 
    /// After we `pop` top element we get following movement:
    ///
    /// ```text
    ///      o -> 1                     4 <<                      2 <- new top
    ///    /   \                      /   \                     /   \
    ///   3     2                    3     2                   3     4 <<
    ///  / \   / \                  / \   /                   / \   / 
    /// 6   9 5   4 <<             6   9 5                   6   9 5  
    ///
    ///    step 1.                    step 2.                    step 3.
    /// ```
    ///
    /// Parent-child relationship balanced itself from top to down and **2** 
    /// became a new top (prioritized) element.
    ///
    /// # Time Complexity
    ///
    /// Worst case is ***O(log(n))***.
    ///
    pub fn pop(&mut self) -> Option<(S, T)> {
        if self.len > 0 {
            let last_ = self.len - 1;
            // If any of the scores is uncomparable move it to the back
            if self.len > 1 && self[0].0.partial_cmp(&self[0].0).is_none() {
                self.swap(0, last_);
            }

            unsafe {
                let _top = ptr::read(self.ptr());
                let _tmp = ptr::read(self.ptr().add(self.len - 1));
                
                // SAFETY: this is safe because the last element will written
                //      in-place of the first element in an allocated space.
                ptr::write(self.ptr(), _tmp);

                self.len -= 1;
                
                if self.len > 1 { self.heapify_down(0); }
                if self.cap() > 1_000 && self.cap() / 4 >= self.len {
                    self.data.shrink();
                }
                Some(_top)
            }
        } else { None }
    }

    /// Check what is a top element in `PriorityQueue`, by getting the reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::PriorityQueue;
    /// 
    /// let mut pq: PriorityQueue<u8, String> = PriorityQueue::new();
    /// assert!(pq.peek().is_none());
    ///
    /// pq.put(1, String::from("Ruiner"));
    /// pq.put(3, String::from("Bazel"));
    /// pq.put(2, String::from("Jho"));
    /// assert_eq!(3, pq.len());
    /// assert_eq!("Ruiner", pq.peek().unwrap().1);
    /// assert_eq!(3, pq.len());
    /// ```
    ///
    /// If `PriorityQueue` is empty it will return `None`.
    ///
    /// # Time Complexity
    ///
    /// `peek`-ing is done in a constant time ***O(1)***
    ///
    pub fn peek(&self) -> Option<&(S, T)> {
        if !self.is_empty() {
            Some(&self[0])
        } else { None }
    }

    /// Returns the number of elements in the `PriorityQueue`
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    /// assert_eq!(0, pq.len());
    ///
    /// pq.put(1, 99);
    /// assert_eq!(1, pq.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` is there are no elements in `PriorityQueue`
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::PriorityQueue;

    /// let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    /// assert!(pq.is_empty());
    ///
    /// pq.put(1, 99);
    /// assert!(!pq.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Remove all the elements from `PriorityQueue`
    ///
    /// # Example
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
    /// assert!(!pq.is_empty());
    ///
    /// pq.clear();
    /// assert!(pq.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.drain(..);
    }

    /// Clears the priority queue, returning iterator over the removed elements
    /// returned items will NOT be in a sorted order. Method takes range as an 
    /// argument.
    ///
    /// # Example
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let mut pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44), (7, 77)]);
    /// assert!(!pq.is_empty());
    ///
    /// // drain everything starting from index 2 till the end.
    /// let mut res: PriorityQueue<usize, usize> = pq.drain(2..).collect();
    /// assert!(pq.is_empty());
    /// assert_eq!(2, res.len());
    ///
    /// // drain the remaining priority queue by giving it full range (..) arg.
    /// res.drain(..);
    /// assert!(res.is_empty());
    /// ```
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, S, T>
    where 
        R: RangeBounds<usize>,
    {
        let len = self.len();
        let Range { start, end } = slice::range(range, ..len);

        // SAFETY: we are reading from row memory within a range from start to 
        //      the `len` where `len` we know is within a memory space of this 
        //      priority queue.
        unsafe {
            let range_slice = slice::from_raw_parts_mut(
                self.as_mut_ptr().add(start), end - start);

            let iter = RawPQIter::new(range_slice);

            // SAFETY: we set up `len` to zero so even if method panics, memory
            //      leak will never happen.
            self.len = 0;

            Drain {
                pq: marker::PhantomData,
                iter,
            }
        }
    }

    /// Clears the priority queue and returns `Vec` with elements in a 
    /// sorted order.
    ///
    /// # Example
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
    ///
    /// let mut res = pq.into_sorted_vec(); 
    /// assert_eq!(3, res.len());
    ///
    /// // we'll be `pop`-ing values from the back of the vector.
    /// // this means highest scoring will be all the way back into the `Vec`
    /// assert_eq!(55, res.pop().unwrap().1);
    /// assert_eq!(44, res.pop().unwrap().1);
    /// assert_eq!(11, res.pop().unwrap().1);
    /// ```
    ///
    /// If priority queue has NAN values they will be sorted after all 
    /// comparable scores without any particular order
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<f32, isize> = PriorityQueue::new();
    /// pq.put(1.1, 10);
    /// pq.put(f32::NAN, -1);
    /// pq.put(2.2, 20);
    /// pq.put(3.3, 30);
    /// pq.put(f32::NAN, -3);
    /// pq.put(4.4, 40);
    /// let res = pq.into_sorted_vec();
    ///
    /// assert_eq!(10, res[0].1);
    /// assert_eq!(20, res[1].1);
    /// assert_eq!(30, res[2].1);
    /// assert_eq!(40, res[3].1);
    /// assert!(res[4].1 < 0 && res[4].1 > -4);
    /// assert!(res[5].1 < 0 && res[5].1 > -4);
    /// ```
    ///
    /// # Time
    /// 
    /// This method drains priority queue into vector and sorts in 
    /// ***O(log(n))*** time.
    pub fn into_sorted_vec(mut self) -> Vec<(S, T)> {
        let mut res: Vec<(S, T)> = self.drain(..)
                                       .collect();

        res.sort_by(|a, b| {
            match a.0.partial_cmp(&b.0) {
                Some(r) => r,
                None => Ordering::Less,
            }
        });
        res
    }

    /// Reduce the length of a priority queue by keeping the first `len` 
    /// elements and dropping the rest.
    ///
    /// If you pass `len` greater than the length of a priority queue this 
    /// will have no effect.
    ///
    /// # Example
    /// 
    /// truncate to keep the first three elements of priority queue.
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let mut pq = PriorityQueue::from(
    ///     [(5, 55), (1, 11), (4, 44), (2, 22), (7, 77), (8, 88)]
    /// );
    ///
    /// pq.truncate(3);
    /// assert_eq!(3, pq.len());
    /// assert_eq!(11, pq.peek().unwrap().1);
    /// ```
    ///
    /// If we try to truncate with `len` > the length of a priority queue:
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let mut pq = PriorityQueue::from([(4, 44), (2, 22), (7, 77), (8, 88)]);
    /// pq.truncate(5);
    ///
    /// assert_eq!(4, pq.len());
    /// ```
    pub fn truncate(&mut self, len: usize) {
        if len > self.len {
            return
        }

        // SAFETY: concerns are none for this because:
        //
        //    * `len` passed to the method is less than self.len so no invalid 
        //        slice can be created.
        //    * self.len is reduced to a new size before `drop_in_place` is called.
        //        no double free error in case `drop_in_place` panics.
        unsafe {
            let remaining = self.len - len;
            let s_ = ptr::slice_from_raw_parts_mut(
                self.as_mut_ptr().add(len), remaining);
            self.len = len;
            ptr::drop_in_place(s_);
        }
    }

    pub fn merge(&mut self, pq: &mut PriorityQueue<S ,T>) {
        while !pq.is_empty() {
            let elem = pq.pop().unwrap();
            self.put(elem.0, elem.1);
        }
    }

    /// Provides the raw pointer to the contiguous block of memory of data
    #[inline]
    fn ptr(&self) -> *mut (S, T) {
        self.data.ptr.as_ptr()
    }

    #[inline]
    /// Provides what's the current capacity of a underlying array
    fn cap(&self) -> usize {
        self.data.cap
    }

    /// Generates the index of a left child (if any) of a item on a given index
    #[inline]
    fn left_child(&self, index: usize) -> usize {
        2 * index + 1
    }

    /// Generates the index of a right child (if any) of a item on a given index
    #[inline]
    fn right_child(&self, index: usize) -> usize {
        2 * index + 2
    }

    /// Generates the index of a parent item (if any) of a item on a given index
    #[inline]
    fn parent(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    /// Checks if given item on provided index has a left child
    #[inline]
    fn has_left(&self, index: usize) -> bool {
        self.left_child(index) < self.len
    }

    /// Checks if given item on provided index has a right child
    #[inline]
    fn has_right(&self, index: usize) -> bool {
        self.right_child(index) < self.len
    }

    /// After item is `pop`-ed this methods helps to balance remaining values
    /// so the prioritized item remains as a root.
    #[inline]
    fn heapify_up(&mut self, index: usize) {
        if index > 0 {
            let parent_ = self.parent(index);
            if self[parent_].0 > self[index].0 {
                self.swap(parent_, index);
                self.heapify_up(parent_);
            }
        }
    }

    /// Store inserted value into a proper position to maintain the balanced
    /// order of parent child relationships and prioritized item as a root.
    #[inline]
    fn heapify_down(&mut self, index: usize) {
        let _left = self.left_child(index);
        let _right = self.right_child(index);
        let mut min_ = index;
        if self.has_left(index) && self[_left].0 < self[min_].0 {
            min_ = _left;
        }
        if self.has_right(index) && self[_right].0 < self[min_].0 {
            min_ = _right;
        }
        if min_ != index {
            self.swap(index, min_);
            self.heapify_down(min_);
        }
    }
}

impl<S, T> Default for PriorityQueue<S, T>
where
    S: PartialOrd,
{
    #[inline]
    fn default() -> Self {
        PriorityQueue::new()
    }
}

impl<S, T> Drop for PriorityQueue<S, T>
where
    S: PartialOrd,
{
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<S, T> Deref for PriorityQueue<S, T>
where
    S: PartialOrd,
{
    type Target = [(S, T)];
    fn deref(&self) -> &[(S, T)] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<S, T> DerefMut for PriorityQueue<S, T>
where
    S: PartialOrd,
{
    fn deref_mut(&mut self) -> &mut [(S, T)] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

impl<S, T> From<Vec<(S, T)>> for PriorityQueue<S, T>
where 
    S: PartialOrd,
{
    /// Create `PriorityQueue` from a `Vec` 
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let vec = vec![(5, 55), (4, 44), (2, 22), (3, 33)];
    /// let mut pq = PriorityQueue::from(vec);
    /// assert_eq!(4, pq.len());
    /// assert_eq!(22, pq.pop().unwrap().1);
    /// ```
    fn from(other: Vec<(S, T)>) -> Self {
        let len = other.len();
        let _cap = rawpq::MIN_CAPACITY;
        match mem::size_of::<(S, T)>() {
            0 => assert!(len < rawpq::MAX_ZST_CAPACITY, "Capacity Overflow"),
            _ => {
                let min_cap = cmp::max(rawpq::MIN_CAPACITY, len) + 1;
                let _cap = cmp::max(min_cap, other.capacity())
                    .next_power_of_two();
            }
        }

        let mut pq: PriorityQueue<S, T> = PriorityQueue::with_capacity(_cap);
        other.into_iter()
             .for_each(|(s, e)| pq.put(s, e));
        pq
    }
}

impl<S, T, const N: usize> From<[(S, T); N]> for PriorityQueue<S, T>
where 
    S: PartialOrd,
{
    /// Create `PriorityQueue` from a slice
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let pq = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
    /// assert_eq!(3, pq.len());
    /// assert_eq!(11, pq.peek().unwrap().1);
    /// ```
    fn from(arr: [(S, T); N]) -> Self {
        let mut pq: PriorityQueue<S, T> = PriorityQueue::with_capacity(N);
        if mem::size_of::<(S, T)>() != 0 {
            arr.into_iter()
               .for_each(|(s, e)| pq.put(s, e));
        }
        pq
    }
}

impl<S, T> FromIterator<(S, T)> for PriorityQueue<S, T>
where 
    S: PartialOrd
{
    /// Or you can create `PriorityQueue` from any iterable collection. This 
    /// also allows to use `collect` method to collect iterable elements into 
    /// a `PriorityQueue`. This allows to build a PriorityQueuety collection 
    /// through `into_iter`.
    ///
    /// # Example
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// // build an iterator
    /// let iter = (0..5).into_iter()
    ///                  .map(|i| (i, i * 2));
    /// 
    /// // create a priority queue from it
    /// let pq = PriorityQueue::from_iter(iter);
    ///
    /// assert_eq!(5, pq.len());
    /// assert_eq!(0, pq.peek().unwrap().1);
    /// ```
    ///
    /// Building priority queue while iterating over values with `collect`
    ///
    /// ```
    /// use priq::PriorityQueue;
    ///
    /// let pq: PriorityQueue<_, _> = (1..6).into_iter()
    ///                                     .map(|i| (i, i + i))
    ///                                     .collect();
    /// assert_eq!(5, pq.len());
    /// assert_eq!(1, pq.peek().unwrap().0);
    /// ```
    fn from_iter<I: IntoIterator<Item = (S, T)>>(iter: I) -> Self {
        let mut pq_ = PriorityQueue::new();
        iter.into_iter()
            .for_each(|(s, e)| pq_.put(s, e));
        pq_
    }
}

impl<S, T> Clone for PriorityQueue<S, T>
where 
    S: PartialOrd
{
    fn clone(&self) -> Self {
        let mut dst = PriorityQueue::<S, T>::with_capacity(self.len + 1);
        // SAFETY: precondition ensures the source is aligned and valid,
        //      and creating `with_capacity` ensures there is enough memory
        //      allocated for a copy priority queue.
        unsafe {
            ptr::copy(self.ptr(), dst.as_mut_ptr(), self.len);
        }

        // SAFETY: we created cloned priority queue with this capacity 
        //      so we update `len` of it.
        dst.len = self.len;
        dst
    }
}

impl<S, T> Add for PriorityQueue<S, T>
where 
    S: PartialOrd
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res: PriorityQueue<S, T> = self;
        let mut rhs_ = rhs;
        res.merge(&mut rhs_);
        res
    }
}

pub struct IntoIter<S, T> {
    _buf: RawPQ<S, T>,
    iter: RawPQIter<S, T>,
}

impl<S, T> Iterator for IntoIter<S, T> {
    type Item = (S, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<S, T> Drop for IntoIter<S, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<S, T> IntoIterator for PriorityQueue<S, T>
where 
    S: PartialOrd + Clone
{
    type Item = (S, T);
    type IntoIter = IntoIter<S, T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let iter = RawPQIter::new(&self);
            let _buf = ptr::read(&self.data);
            mem::forget(self);

            IntoIter { iter, _buf, }
        }
    }
}

struct RawPQIter<S, T> {
    start: *const (S, T),
    end: *const (S, T),
}

impl<S, T> RawPQIter<S, T> {
    unsafe fn new(slice: &[(S, T)]) -> Self {
        RawPQIter {
            start: slice.as_ptr(),
            end: if mem::size_of::<(S, T)>() == 0 {
                ((slice.as_ptr() as usize) + slice.len()) as *const _
            } else if slice.is_empty() {
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            }
        }
    }
}

impl<S, T> Iterator for RawPQIter<S, T> {
    type Item = (S, T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let res = ptr::read(self.start);
                self.start = match mem::size_of::<(S, T)>() {
                    0 => (self.start as usize + 1) as *const _,
                    _ => self.start.offset(1),
                };
                Some(res)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end as usize - self.start as usize;
        match mem::size_of::<(S, T)>() {
            0 => (len, Some(len)),
            i => (len / i, Some(len / i)),
        }
    }
}

pub struct Drain<'a, S: 'a, T: 'a>
where 
    S: PartialOrd,
{
    pq: marker::PhantomData<&'a mut PriorityQueue<S, T>>,
    iter: RawPQIter<S, T>,
}

impl<'a, S, T> Iterator for Drain<'a, S, T>
where 
    S: PartialOrd,
{
    type Item = (S, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, S, T> Drop for Drain<'a, S, T>
where 
    S: PartialOrd,
{
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}
