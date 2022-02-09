//! Array implementation of the min/max heap.
//!
//! `PriorityQueue` is build on top of raw vector for efficient performance.
//!
//! There are two major reasons what makes this `PriorityQueue` different from
//! other binary heap implementations currently available:
//!
//! 1 - Allows data ordering to scores with `PartialOrd`.
//!     - Every other min-max heap requires 
//!     [total ordering](https://bit.ly/3GCWvYL) of scores (e.g. should 
//!     implement `Ord` trait). This can be an issue, for example, when you 
//!     want to order items based on a float scores, which doesn't implement
//!     `Ord` trait.
//!     - You can read about Rust's implementation or `Ord`, `PartialOrd` and 
//!     what's the different [here](https://bit.ly/3J7NwQI)
//!
//! 2 - Separation of score and item you wish to store.
//!     - This frees enforcement for associated items to implement any ordering.
//!     - Makes easier to evaluate items' order.
//!
//! 3 - Equal scoring items are stored at first available free space.
//!     - This gives performance boost for large number of entries.
//!
//! 4 - Easy to use!
//!
//! You can read more about this crate on [my blog](https://www.bexxmodd.com)

use std::mem;
use std::ptr;
use std::ops::{Deref, DerefMut};

use crate::rawpq::RawPQ;

/// A Min-Max Heap with separate arguments for `score` and associated `item`.
///
/// `Default` implementation is a Min-Heap where the top node (root) is the 
/// lowest scoring element:
///
/// <center><p>10<p></center>
/// <center><p>/&emsp;&ensp;\</p></center>
/// <center><p>58&emsp;&emsp;70</p></center>
/// <center><p>/&emsp;\&emsp;&emsp;/&emsp;\</p></center>
/// <center><p>80&emsp;&ensp;92&emsp;97&emsp;&ensp;99</p></center>
///
/// > The value of Parent Node is small than Child Node.
///
/// Every parent node, including the top (root) node, is less than or equal 
/// to the value of its children nodes. And left child is always less than or 
/// equal to right child.
///
/// `PriorityQueue` allows duplicate score/item values. When you [`put`] the 
/// item with the similar score that's already in the queue new entry will be 
/// stored at the first empty location in memory. This gives incremental 
/// performance boost (instead of resolving by using associated item as a 
/// secondary tool to priority evaluation). Also, this form of implementation 
/// doesn't enforce for the item `T` to have any implemented ordering.
/// This guarantees that the top node will always be of minimum value.
///
/// You can initilize an empty `PriorityQueue` and later add items:
///
/// ```
/// use priq::priq::PriorityQueue;
///
/// let pq: PriorityQueue<usize, String> = PriorityQueue::new();
/// ```
///
/// Or you can `heapify` from an `Vec` and/or `slice`:
///
/// ```
/// use priq::priq::PriorityQueue;
///
/// // todo
/// ```
/// 
/// The standard usage of this data structure is to through [`put`] to add to 
/// the queue and [`pop`] to remove the top item, and [`peek`] to check what's 
/// the top item in the queue. Stored structure of the items is a balanced tree 
/// realized using an array with contiguous memory location. This allows to 
/// maintain proper parent child relationship between [`put`]-ed items.
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
/// | [`pop`]   | _O(long(n))_    |
/// | [`peek`]  | _O(1)_          |
///
/// You can also iterate over elements using _for loop_ but the returned slice 
/// will not proper order as heap is re-balanced after each insertion and 
/// deletion. If you want to grab items in a proper priority call [`pop`] in 
/// a loop until it returns `None`.
///
/// What if you want to custom `struct` without having separate and specific score?
/// You can pass the `struct` as a score and as an associated value (In this 
/// case custom `struct` should implement `PartialOrd`):
///
/// ```
/// use priq::priq::PriorityQueue;
///
/// #[derive(PartialOrd, PartialEq, Clone)]
/// struct Wrapper(i32);
///
/// let mut pq: PriorityQueue<Wrapper, Wrapper> = PriorityQueue::new();
/// let a_ = Wrapper(3);
/// pq.put(a_.clone(), a_);
/// ```
///
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
    /// use priq::priq::PriorityQueue;
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

    // /// Create `PriorityQueue` from a slice
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use priq::priq::PriorityQueue;
    // ///
    // /// let pq = PriorityQueue::from([(-1, 1), (-3, 3), (-2, 2)]);
    // /// ```
    // pub fn from(slice: &) -> Self {
    //     todo!()
    // }

    /// If you expect that you'll be putting at least `n` number of items in 
    /// `PriorityQueue` you can create it with space of at least elements equal
    /// to `cap`. This can boost the performance for the large number of sets, 
    /// because it will eliminate the need to grow underlying array more often.
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::priq::PriorityQueue;
    ///
    /// let pq: PriorityQueue<usize, usize> = PriorityQueue::with_capacity(100);
    /// ```
    pub fn with_capacity(cap: usize) -> Self {
        PriorityQueue {
            data: RawPQ::with_capacity(cap),
            len: 0,
        }
    }

    /// Returns the number of elements in the `PriorityQueue`
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::priq::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    /// assert_eq!(0, pq.len());
    ///
    /// pq.put(1, 99);
    /// assert_eq!(1, pq.len());
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` is there are no elements in `PriorityQueue`
    ///
    /// # Examples
    ///
    /// ```
    /// use priq::priq::PriorityQueue;

    /// let mut pq: PriorityQueue<usize, usize> = PriorityQueue::new();
    /// assert!(pq.is_empty());
    ///
    /// pq.put(1, 99);
    /// assert!(!pq.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn put(&mut self, score: S, item: T) {
        if self.cap() == self.len { self.data.grow(); }
        self.len += 1;
        let _entry = (score, item);
        unsafe {
            ptr::write(self.ptr().add(self.len - 1), _entry);
        }
        self.heapify_up(self.len - 1);
    }

    pub fn pop(&mut self) -> Option<(S, T)> {
        if self.len > 0 {
            unsafe {
                let _top = ptr::read(self.ptr());
                let _tmp = ptr::read(self.ptr().add(self.len - 1));
                ptr::write(self.ptr(), _tmp);
                self.len -= 1;
                
                if self.len > 1 { self.heapify_down(0); }
                Some(_top)
            }
        } else { None }
    }

    pub fn peek(&self) -> Option<&(S, T)> {
        if !self.is_empty() {
            unsafe {
                ptr::read(&self.ptr().as_ref())
            }
        } else { None }
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

    /// Swaps two values of provided indices in memory
    #[inline]
    fn swap(&mut self, i: usize, j: usize) {
        unsafe {
            let _a = ptr::read(&self.ptr().add(i));
            let _b = ptr::read(&self.ptr().add(j));
            ptr::swap(_a, _b);
        }
    }

    /// After item is `pop`-ed this methods helps to balance remaining values
    /// so the prioritized item remains as a root.
    fn heapify_up(&mut self, index: usize) {
        if index > 0 {
            let parent_ = self.parent(index);
            let res = ptr::slice_from_raw_parts(self.ptr(), self.len);
            if unsafe { (&*res)[parent_].0 > (&*res)[index].0 } {
                self.swap(parent_, index);
                self.heapify_up(parent_);
            }
        }
    }

    /// Store inserted value into a proper position to maintain the balanced
    /// order of parent child relationships and prioritized item as a root.
    fn heapify_down(&mut self, index: usize) {
        let _left = self.left_child(index);
        let _right = self.right_child(index);
        let mut min_ = index;
        unsafe {
            let heap_ = &*ptr::slice_from_raw_parts(self.ptr(), self.len);
            if self.has_left(index) && heap_[_left].0 < heap_[min_].0 {
                min_ = _left;
            }
            if self.has_right(index) && heap_[_right].0 < heap_[min_].0 {
                min_ = _right;
            }
            if min_ != index {
                self.swap(index, min_);
                self.heapify_down(min_);
            }
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

// impl<S, T> Deref for PriorityQueue<S, T>
// where
//     S: PartialOrd,
// {
//     type Target = [(S, T)];
//     fn deref(&self) -> &[(S, T)] {
//         unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
//     }
// }
// 
// impl<S, T> DerefMut for PriorityQueue<S, T>
// where
//     S: PartialOrd,
// {
//     fn deref_mut(&mut self) -> &mut [(S, T)] {
//         unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
//     }
// }

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
