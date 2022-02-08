use std::cmp;
use std::mem;
use std::fmt::Display;
use std::ptr;
use std::marker::PhantomData;
use std::alloc::{self, Layout};
use std::ops::{Deref, DerefMut};

struct RawPQ<S, T> {
    ptr: ptr::NonNull<(S, T)>,
    cap: usize,
    _marker: PhantomData<(S, T)>,
}

unsafe impl<T: Send, S: Send> Send for RawPQ<S, T> {}
unsafe impl<T: Sync, S: Sync> Sync for RawPQ<S, T> {}

impl<S, T> RawPQ<S,T> {
    fn new() -> Self {
        let cap = if mem::size_of::<(S, T)>() == 0 { !0 } else { 0 };
        RawPQ {
            ptr: ptr::NonNull::dangling(),
            cap,
            _marker: PhantomData,
        }
    }

    fn grow(&mut self) {
        assert_ne!(mem::size_of::<(S, T)>(), 0, "Capacity Overflow");

        let (new_cap, new_layout) = match self.cap {
            0 => (4, Layout::array::<(S, T)>(4).unwrap()),
            _ => {
                let new_cap = 3 * self.cap;
                let new_layout = Layout::array::<(S, T)>(new_cap).unwrap();
                (new_cap, new_layout)
            }
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = match self.cap {
            0 => unsafe { alloc::alloc(new_layout) },
            _ => {
                let old_layout = Layout::array::<(S, T)>(self.cap).unwrap();
                let old_ptr = self.ptr.as_ptr() as *mut u8;
                unsafe {
                    alloc::realloc(old_ptr, old_layout, new_layout.size())
                }
            }
        };

        self.ptr = match ptr::NonNull::new(new_ptr as *mut (S, T)) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<S, T> Drop for RawPQ<S, T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<(S, T)>();
        if self.cap != 0 && elem_size != 0 {
            unsafe {
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<(S, T)>(self.cap).unwrap(),
                )
            }
        }
    }
}


pub struct PriorityQueue<S, T> 
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    data: RawPQ<S, T>,
    len: usize,
}


impl<S, T> PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    fn ptr(&self) -> *mut (S, T) {
        self.data.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.data.cap
    }

     pub fn new() -> Self {
         PriorityQueue {
             data: RawPQ::new(),
             len: 0,
         }
     }

     pub fn len(&self) -> usize {
         self.len
     }
 
     pub fn is_empty(&self) -> bool {
         self.len == 0
     }

    // TODO: Implement insertion algorithm to maintain min binary heap
    // Runtime complexity should be O(log(n))
    pub fn put(&mut self, score: S, item: T) {
        println!("Cap: {} .... Len: {}", self.cap(), self.len);
        if self.cap() - self.len <= 1 { self.data.grow(); }
        self.len += 1;
        self.percolate_down(self.len, (score, item));
    }

    // pub fn pop(&mut self) -> Option<(S, T)> {
    //     if self.len > 1 {
    //         self.len -= 1;
    //         unsafe {
    //             let _top = ptr::read(self.ptr().as_ptr().add(1));
    //             let _bottom = ptr::read(self.ptr().as_ptr().add(self.len));
    //             self.percolate_down(1, _bottom);
    //             Some(_top)
    //         }
    //     } else { None }
    // }

    pub fn peek(&self) -> Option<&(S, T)> {
        if !self.is_empty() {
            unsafe {
                ptr::read(&self.ptr().add(1).as_ref())
            }
        } else { None }
    }

    // TODO: Debug the logic
    fn percolate_down(&mut self, index: usize, entry: (S, T)) {
        let i_2 = if index == 0 { 2 } else { (index + 1) * 2 };
        let res = ptr::slice_from_raw_parts(self.ptr(), self.len + 1);
        unsafe {
            match (i_2).cmp(&(self.len)) {
                cmp::Ordering::Less => {
                    println!("Less: {} index: {}", entry.1, index);
                    let j = if (&*res)[i_2].0 < (&*res)[i_2 + 1].0 { i_2 }
                            else { i_2 + 1 };

                    if (&*res)[j].0 < entry.0 {
                        let tmp_ = ptr::read(self.ptr().add(j));
                        ptr::write(self.ptr().add(index), tmp_);
                        self.percolate_down(j, entry);
                    } else {
                        ptr::write(self.ptr().add(index), entry);
                    }
                },
                cmp::Ordering::Equal => {
                    println!("Equal: {} index: {}", entry.1, index);
                    if (&*res)[i_2].0 < entry.0 {
                        let tmp_ = ptr::read(self.ptr().add(i_2));
                        ptr::write(self.ptr().add(index), tmp_);
                        ptr::write(self.ptr().add(i_2), entry);
                    } else {
                        ptr::write(self.ptr().add(index), entry);
                    }
                },
                _ => {
                    println!("Less: {} index: {}", entry.1, index);
                    ptr::write(self.ptr().add(index), entry);
                }
            }
        };
    }

//     fn percolate_up(&mut self, index: usize, entry: (S, T)) {
//         if index == 0 { return self.percolate_up(1, entry) }
// 
//         let res = ptr::slice_from_raw_parts(self.entries.as_ptr(), self.len);
//         if index == 1 {
//             unsafe {
//                 ptr::write(self.entries.as_ptr().add(1), entry);
//             }
//         } else if unsafe { &*res }[index / 2].0 < entry.0 {
//             unsafe {
//                 ptr::write(self.entries.as_ptr().add(index), entry);
//             }
//         } else {
//             unsafe {
//                 let tmp_ = ptr::read(self.entries.as_ptr().add(index / 2));
//                 ptr::write(self.entries.as_ptr().add(index), tmp_);
//             }
//             self.percolate_up(index / 2, entry);
//         }
//     }

    pub fn print(&mut self) {
        let res = ptr::slice_from_raw_parts(self.ptr(), self.len);
        unsafe {
            for i in 0..self.len() {
                println!("{}: Value {}", i, (&*res)[i].1);
            }
        }
    }
}

// unsafe impl<T: Send + Display + Clone, S: Send + PartialOrd + Display>
//     Send for PriorityQueue<S, T> {}
// unsafe impl<T: Sync + Display + Clone, S: Sync + PartialOrd + Display>
//     Sync for PriorityQueue<S, T> {}

// pub struct IntoIter<S, T>
// where
//     S: PartialOrd + Display,
//     T: Clone + Display
// {
//     buf: ptr::NonNull<(S, T)>,
//     cap: usize,
//     start: *const (S, T),
//     end: *const (S, T),
//     _marker: PhantomData<(S, T)>,
// }
// 
// impl<S, T> IntoIterator for PriorityQueue<S, T>
// where
//     S: PartialOrd + Display,
//     T: Clone + Display
// {
//     type Item = (S, T);
//     type IntoIter = IntoIter<S, T>;
// 
//     fn into_iter(self) -> IntoIter<S, T> {
//         let entries = self.entries;
//         let cap = self.cap;
//         let len = self.len;
// 
//         mem::forget(self);
//         unsafe {
//             IntoIter {
//                 buf: entries,
//                 cap,
//                 start: entries.as_ptr().add(1),
//                 end: match cap {
//                     0|1 => entries.as_ptr(),
//                     _ => entries.as_ptr().add(len + 1),
//                 },
//                 _marker: PhantomData,
//             }
//         }
//     }
// }
// 
// impl<S, T> Iterator for IntoIter<S, T>
// where
//     S: PartialOrd + Display,
//     T: Clone + Display
// {
//     type Item = (S, T);
//     fn next(&mut self) -> Option<(S, T)> {
//         if self.start == self.end {
//             None
//         } else {
//             unsafe {
//                 let res = ptr::read(self.start);
//                 self.start = self.start.offset(1);
//                 Some(res)
//             }
//         }
//     }
// 
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let len = (self.end as usize - self.start as usize)
//             / mem::size_of::<(S, T)>();
//         (len, Some(len))
//     }
// 
// }

impl<S, T> Default for PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    fn default() -> Self {
        PriorityQueue::new()
    }
}

// impl<S, T> Drop for PriorityQueue<S, T>
// where
//     S: PartialOrd + Display,
//     T: Clone + Display
// {
//     fn drop(&mut self) {
//         if self.cap  != 0 {
//             while self.pop().is_some() {  }
//             let layout = Layout::array::<(S, T)>(self.cap).unwrap();
//             unsafe {
//                 alloc::dealloc(self.entries.as_ptr() as *mut u8, layout);
//             }
//         }
//     }
// }
// 
// impl<S, T> Drop for IntoIter<S, T>
// where
//     S: PartialOrd + Display,
//     T: Clone + Display
// {
//     fn drop(&mut self) {
//         if self.cap  != 0 {
//             for _ in &mut *self {}
//             let layout = Layout::array::<(S, T)>(self.cap).unwrap();
//             unsafe {
//                 alloc::dealloc(self.buf.as_ptr() as *mut u8, layout);
//             }
//         }
//     }
// }
// 
// impl<S, T> Deref for PriorityQueue<S, T>
// where
//     S: PartialOrd + Display,
//     T: Clone + Display
// {
//     type Target = [(S, T)];
//     fn deref(&self) -> &[(S, T)] {
//         unsafe {
//             std::slice::from_raw_parts(self.entries.as_ptr(), self.len)
//         }
//     }
// }
// 
// impl<S, T> DerefMut for PriorityQueue<S, T>
// where
//     S: PartialOrd + Display,
//     T: Copy + Clone + Display
// {
//     fn deref_mut(&mut self) -> &mut [(S, T)] {
//         unsafe {
//             std::slice::from_raw_parts_mut(self.entries.as_ptr(), self.len)
//         }
//     }
// }
// 
// 
