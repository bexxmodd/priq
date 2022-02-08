use std::mem;
use std::ptr;
use std::marker;
use std::alloc;


const INITIAL_CAPACITY: usize = 10;
const MAX_ZST_CAPACITY: usize = 1 << (usize::BITS - 1);

pub struct RawPQ<S, T> {
    pub ptr: ptr::NonNull<(S, T)>,
    pub cap: usize,
    _marker: marker::PhantomData<(S, T)>,
}

unsafe impl<T: Send, S: Send> Send for RawPQ<S, T> {}
unsafe impl<T: Sync, S: Sync> Sync for RawPQ<S, T> {}

impl<S, T> RawPQ<S,T> {
    pub fn new() -> Self {
        let cap = match mem::size_of::<(S, T)>() {
            0 => MAX_ZST_CAPACITY,
            _ => 0,
        };

        RawPQ {
            ptr: ptr::NonNull::dangling(),
            cap,
            _marker: marker::PhantomData,
        }
    }

    pub fn grow(&mut self) {
        assert_ne!(mem::size_of::<(S, T)>(), 0, "Capacity Overflow");

        let (new_cap, new_layout) = match self.cap {
            0 => (INITIAL_CAPACITY,
                alloc::Layout::array::<(S, T)>(INITIAL_CAPACITY).unwrap()),
            _ => {
                let new_cap = 3 * self.cap;
                let new_layout = alloc::Layout::array::<(S, T)>(new_cap)
                                    .unwrap();
                (new_cap, new_layout)
            }
        };

        assert!(
            new_layout.size() <= MAX_ZST_CAPACITY, "Allocation is too large"
        );
        let new_ptr = match self.cap {
            0 => unsafe { alloc::alloc(new_layout) },
            _ => {
                let old_layout = alloc::Layout::array::<(S, T)>(self.cap)
                                    .unwrap();
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
                    alloc::Layout::array::<(S, T)>(self.cap).unwrap(),
                )
            }
        }
    }
}
