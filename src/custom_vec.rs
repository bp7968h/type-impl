use std::ptr::NonNull;
use std::{alloc, mem};

pub struct CustomVec<T: Sized> {
    ptr: Option<NonNull<T>>,
    len: usize,
    capacity: usize,
}

impl<T: Sized> CustomVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: None,
            len: 0,
            capacity: 0,
        }
    }
    
    pub fn push(&mut self, item: T) {
        match self.capacity() {
            0 => {
                let layout = alloc::Layout::array::<T>(4).expect("Allocation failed");
                let ptr = unsafe { alloc::alloc(layout) } as *mut T;
                let ptr = NonNull::new(ptr).expect("Could not allocate");
                unsafe { ptr.as_ptr().write(item)};
            
                self.ptr = Some(ptr);
                self.capacity = 4;
                self.len = 1;
            },
            c if c > self.len() => {
                if let Some(ptr) = self.ptr {
                    unsafe {
                        ptr.as_ptr().add(self.len()).write(item);
                    }
                    self.increament_len();
                }
            },
            _ => {
                debug_assert!(self.len() == self.capacity());
                if let Some(ptr) = self.ptr {
                    let new_capacity = self.capacity().checked_mul(2).expect("Capacity wrapped");
                    let size = mem::size_of::<T>() * self.capacity();
                    let align = mem::align_of::<T>();
                    let align = size.checked_add(size % align).expect("Cannot allocate");
                    let new_ptr = unsafe {
                        let layout = alloc::Layout::from_size_align_unchecked(size, align);
                        let new_size = mem::size_of::<T>() * new_capacity;
                        let new_ptr  = alloc::realloc(ptr.as_ptr() as *mut u8, layout, new_size);
                        let new_ptr = NonNull::new(new_ptr as *mut T).expect("Could not reallocate");
                        new_ptr.as_ptr().add(self.len()).write(item);
                        new_ptr
                    };
                    
                    self.ptr = Some(new_ptr);
                    self.increament_len();
                    self.set_capacity(new_capacity);
                }
            }
        }
        
    }
    
    fn increament_len(&mut self) {
        self.len += 1;
    }
    
    fn set_capacity(&mut self, new_cap: usize) {
        self.capacity = new_cap;
    }
    
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len() {
            return None;
        }
        if let Some(ptr) = self.ptr {
            return Some(unsafe { &*ptr.as_ptr().add(idx) })
        }
        
        None
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Sized> Drop for CustomVec<T> {
    fn drop(&mut self) {
        if let Some(mut ptr) = self.ptr {
            let len = self.len();
            let size = mem::size_of::<T>() * self.capacity();
            let align = mem::align_of::<T>();

            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                std::ptr::drop_in_place(std::slice::from_raw_parts_mut(&mut ptr as *mut _, len));
                alloc::dealloc(ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}
