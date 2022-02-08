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

    pub fn pop(&mut self) -> Option<(S, T)> {
        if self.len > 0 {
            unsafe {
                let _top = ptr::read(self.ptr());
                let _tmp = ptr::read(self.ptr().add(self.len - 1));
                ptr::write(self.ptr(), _tmp);
                self.len -= 1;
                
                if self.len > 1 {
                    self.heapify_down(0);
                }
                Some(_top)
            }
        } else { None }
    }

    fn heapify_down(&mut self, index: usize) {
        let _left = self.left_child(index);
        let _right = self.right_child(index);
        let heap_ = ptr::slice_from_raw_parts(self.ptr(), self.len);

        unsafe {
            let min_ = if self.has_left(index) &&
                (&*heap_)[_left].0 < (&*heap_)[index].0 {
                _left
            } else if self.has_right(index) &&
                (&*heap_)[_right].0 < (&*heap_)[index].0 {
                _right
            } else {
                index
            };

            if min_ != index {
                self.swap(index, min_);
                self.heapify_down(min_);
            }
        }
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

    pub fn peek(&self) -> Option<&(S, T)> {
        if !self.is_empty() {
            unsafe {
                ptr::read(&self.ptr().as_ref())
            }
        } else { None }
    }

    fn ptr(&self) -> *mut (S, T) {
        self.data.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.data.cap
    }

    fn left_child(&self, index: usize) -> usize {
        2 * index + 1
    }

    fn right_child(&self, index: usize) -> usize {
        2 * index + 2
    }

    fn parent(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    fn has_left(&self, index: usize) -> bool {
        self.left_child(index) < self.len
    }

    fn has_right(&self, index: usize) -> bool {
        self.right_child(index) < self.len
    }

    fn swap(&mut self, i: usize, j: usize) {
        unsafe {
            let _a = ptr::read(&self.ptr().add(i));
            let _b = ptr::read(&self.ptr().add(j));
            ptr::swap(_a, _b);
        }
    }

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

    pub fn print(&mut self) {
        let res = ptr::slice_from_raw_parts(self.ptr(), self.len);
        unsafe {
            for i in 0..self.len {
                println!("{}: Value {}", i, (&*res)[i].1);
            }
        }
    }
}

impl<S, T> Default for PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    fn default() -> Self {
        PriorityQueue::new()
    }
}

impl<S, T> Drop for PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<S, T> Deref for PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    type Target = [(S, T)];
    fn deref(&self) -> &[(S, T)] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<S, T> DerefMut for PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Copy + Clone + Display
{
    fn deref_mut(&mut self) -> &mut [(S, T)] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
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
