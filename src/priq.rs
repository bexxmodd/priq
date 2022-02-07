use std::mem;
use std::fmt::Display;
use std::ptr;
use std::marker::PhantomData;
use std::alloc::{self, Layout};
use std::ops::{Deref, DerefMut};

pub struct PriorityQueue<S, T> 
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    entries: ptr::NonNull<(S, T)>,
    cap: usize,
    len: usize,
    _marker: (PhantomData<S>, PhantomData<T>),
}

impl<S, T> PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    pub fn new() -> Self {
        assert_ne!(mem::size_of::<T>(), 0, "Can't handle ZSTs");
        PriorityQueue {
            entries: ptr::NonNull::dangling(),
            len: 0,
            cap: 0,
            _marker: (PhantomData, PhantomData),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn put(&mut self, score: S, item: T) {
        if self.cap == self.len { self.grow(); }
        if self.len == 0 { return self.push(score, item) }

        let mut index = self.len + 1;
        let res = ptr::slice_from_raw_parts(self.entries.as_ptr(), self.len);
        loop {
            println!("Index >> {} ::: {}", index, item);
            if index == 0 { break; }
            index = (index - 1) / 2;

            if score <= unsafe{ &*res }[index].0 {
                break
            }

        }
        let _entry = (score, item);
        println!("{} to add at: {}", _entry.1, index);
        unsafe {
            ptr::copy(self.entries.as_ptr().add(index + 1),
                      self.entries.as_ptr().add(index + 2),
                      self.len - index);
            ptr::write(self.entries.as_ptr().add(index), _entry);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<(S, T)> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.entries.as_ptr().add(self.len)))
            }
        }
    }

    pub fn peek(&self) -> Option<&(S, T)> {
        if self.is_empty() {
            None
        } else {
            unsafe {
                let _top = ptr::read(&self.entries.as_ref());
                Some(_top)
            }
        }
    }

    pub fn push(&mut self, score: S, item: T) {
        if self.len == self.cap { self.grow(); }

        let _entry = (score, item);
        unsafe {
            ptr::write(self.entries.as_ptr().add(self.len), _entry);
        }
        self.len += 1;
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (4, Layout::array::<(S, T)>(4).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<(S, T)>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<(S, T)>(self.cap).unwrap();
            let old_entries = self.entries.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_entries, old_layout, new_layout.size()) }
        };

        self.entries = match ptr::NonNull::new(new_ptr as *mut (S, T)) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
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
        if self.cap  != 0 {
            while self.pop().is_some() {  }
            let layout = Layout::array::<(S, T)>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.entries.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<S, T> Deref for PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    type Target = [(S, T)];
    fn deref(&self) -> &[(S, T)] {
        unsafe {
            std::slice::from_raw_parts(self.entries.as_ptr(), self.len)
        }
    }
}

impl<S, T> DerefMut for PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Copy + Clone + Display
{
    fn deref_mut(&mut self) -> &mut [(S, T)] {
        unsafe {
            std::slice::from_raw_parts_mut(self.entries.as_ptr(), self.len)
        }
    }
}

// unsafe impl<T: Send, S: Send + PartialEq + Copy + Clone + Display> Send for PriorityQueue<S, T> {}
// unsafe impl<T: Sync, S: Sync + PartialEq + Copy + Clone + Display> Sync for PriorityQueue<S, T> {}
