use std::{
    alloc::{alloc, Layout},
    ops::Index,
    ptr::{self, NonNull},
};

pub struct Vec<T: Sized> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn resize(&mut self) {
        self.capacity = if self.capacity > 0 {
            self.capacity * 2
        } else {
            8
        };
        let layout = Layout::array::<T>(self.capacity).unwrap();
        unsafe {
            let ptr = alloc(layout);
            self.ptr = NonNull::new_unchecked(ptr.cast());
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.resize()
        }

        unsafe {
            let end = self.ptr.as_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }

    // This was taken directly from std Vec.
    // If I understand it correctly it shifts the value but does not resize
    pub fn remove(&mut self, index: usize) {
        unsafe {
            let ptr = self.ptr.as_ptr().add(index);
            ptr::read(ptr);
            ptr::copy(ptr.add(1), ptr, self.len - index - 1);
        }
        self.len -= 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(self) -> bool {
        self.len == 0
    }

    pub fn new() -> Self {
        Vec {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }
}
impl<T> Index<isize> for Vec<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        if index >= self.len.try_into().unwrap() {
            panic!()
        }
        unsafe {
            let ptr = self.ptr.as_ptr().offset(index);
            NonNull::new_unchecked(ptr.cast()).as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let list = Vec::<i32>::new();
        assert_eq!(list.is_empty(), true)
    }

    #[test]
    fn can_push() {
        let mut list = Vec::<i32>::new();
        list.push(69);
        assert_eq!(list.is_empty(), false)
    }

    #[test]
    fn can_index() {
        let mut list = Vec::<i32>::new();
        list.push(69);
        assert_eq!(list[0], 69)
    }

    #[test]
    fn can_index_2() {
        let mut list = Vec::<i32>::new();
        list.push(69);
        list.push(666);
        assert_eq!(list[1], 666)
    }

    #[test]
    fn get_len() {
        let mut list = Vec::<i32>::new();
        list.push(69);
        list.push(666);
        assert_eq!(list.len(), 2)
    }

    #[test]
    #[should_panic]
    fn can_panic_on_index() {
        let mut list = Vec::<i32>::new();
        list.push(69);
        list.push(666);
        assert_eq!(list[2], 666)
    }

    #[test]
    fn can_remove() {
        let mut list = Vec::<i32>::new();
        list.push(69);
        list.push(666);
        list.push(777);
        list.remove(1);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], 69);
    }

    #[test]
    fn remove_is_not_accessible() {
        let mut list = Vec::<i32>::new();
        list.push(69);
        list.push(666);
        list.push(777);
        list.remove(1);
        assert_eq!(list[1], 777)
    }

    #[test]
    #[should_panic]
    fn remove_is_not_accessible_2() {
        let mut list = Vec::<i32>::new();
        list.push(69);
        list.push(666);
        list.push(777);
        list.remove(1);
        assert_eq!(list[2], 777)
    }
}
