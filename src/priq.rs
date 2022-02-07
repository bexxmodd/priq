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
    _marker: PhantomData<(S, T)>,
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
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // TODO: correct the algorithm to put item using binary search to 
    //  achieve O(log(n)) complexity for insertion and keep O(n) for pop
    pub fn put(&mut self, score: S, item: T) {
        if self.cap == self.len { self.grow(); }

        let mut index = self.len;
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
        self.insert(index, _entry);
        self.len += 1;
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

    fn insert(&mut self, index: usize, entry: (S, T)) {
        unsafe {
            ptr::copy(self.entries.as_ptr().add(index),
                      self.entries.as_ptr().add(index + 1),
                      self.len - index);
            ptr::write(self.entries.as_ptr().add(index), entry);
        }
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

pub struct IntoIter<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    buf: ptr::NonNull<(S, T)>,
    cap: usize,
    start: *const (S, T),
    end: *const (S, T),
    _marker: PhantomData<(S, T)>,
}

impl<S, T> IntoIterator for PriorityQueue<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    type Item = (S, T);
    type IntoIter = IntoIter<S, T>;

    fn into_iter(self) -> IntoIter<S, T> {
        let entries = self.entries;
        let cap = self.cap;
        let len = self.len;

        mem::forget(self);
        unsafe {
            IntoIter {
                buf: entries,
                cap,
                start: entries.as_ptr(),
                end: match cap {
                    0 => entries.as_ptr(),
                    _ => entries.as_ptr().add(len),
                },
                _marker: PhantomData,
            }
        }
    }
}

impl<S, T> Iterator for IntoIter<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    type Item = (S, T);
    fn next(&mut self) -> Option<(S, T)> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let res = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(res)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize)
            / mem::size_of::<(S, T)>();
        (len, Some(len))
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

impl<S, T> Drop for IntoIter<S, T>
where
    S: PartialOrd + Display,
    T: Clone + Display
{
    fn drop(&mut self) {
        if self.cap  != 0 {
            for _ in &mut *self {}
            let layout = Layout::array::<(S, T)>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.buf.as_ptr() as *mut u8, layout);
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



unsafe impl<T: Send + Display + Clone, S: Send + PartialOrd + Display>
    Send for PriorityQueue<S, T> {}
unsafe impl<T: Sync + Display + Clone, S: Sync + PartialOrd + Display>
    Sync for PriorityQueue<S, T> {}
