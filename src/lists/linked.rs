use std::{marker::PhantomData, ops::Index, ptr::NonNull};

struct SingleLinkedListNode<T> {
    value: T,
    next: Option<NonNull<SingleLinkedListNode<T>>>,
}

impl<T> SingleLinkedListNode<T> {
    /// Creates a new [`SingleLinkedListNode<T>`].
    fn new(value: T) -> Self {
        Self {
            value,
            next: Option::None,
        }
    }

    fn add_next(&mut self, item: T) {
        self.next = Option::Some(Box::leak(Box::new(SingleLinkedListNode::new(item))).into());
    }
}

pub struct SingleLinkedList<T> {
    first: Option<NonNull<SingleLinkedListNode<T>>>,
    last: Option<NonNull<SingleLinkedListNode<T>>>,
    len: usize,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            first: Option::None,
            last: Option::None,
            len: 0,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn iter(&self) -> SingleLinkedListIter<T> {
        SingleLinkedListIter {
            current: self.first,
            len: 0,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn append(&mut self, item: T) {
        if self.is_empty() {
            let node = Some(Box::leak(Box::new(SingleLinkedListNode::new(item))).into());
            self.first = node;
            self.last = node;
        } else {
            let last = unsafe { self.last.unwrap().as_mut() };
            last.add_next(item);
        }

        self.len += 1;
    }
}

impl<T> Default for SingleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, Idx> Index<Idx> for SingleLinkedList<T>
where
    Idx: PartialEq<usize>,
    T: Copy,
{
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        let mut i = 0;
        let mut current = self.first;
        loop {
            match current {
                Some(node) => {
                    if index == i {
                        return unsafe { &node.as_ref().value };
                    }
                    current = unsafe { node.as_ref().next };
                    i += 1;
                }
                None => panic!(),
            }
        }
    }
}

pub struct SingleLinkedListIter<'a, T: 'a> {
    current: Option<NonNull<SingleLinkedListNode<T>>>,
    len: usize,
    marker: PhantomData<&'a SingleLinkedListNode<T>>,
}

impl<'a, T> Iterator for SingleLinkedListIter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        match self.current {
            Some(cur) => {
                self.current = unsafe { cur.as_ref().next };
                Some(unsafe { &cur.as_ref().value })
            }
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let list = SingleLinkedList::<i32>::new();
        assert_eq!(list.is_empty(), true)
    }

    #[test]
    fn can_append() {
        let mut list = SingleLinkedList::<i32>::new();
        assert_eq!(list.is_empty(), true);
        list.append(69);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn can_append_multiple() {
        let mut list = SingleLinkedList::<i32>::new();
        assert_eq!(list.is_empty(), true);
        list.append(69);
        list.append(69);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn can_index() {
        let mut list = SingleLinkedList::<i32>::new();
        list.append(69);
        assert_eq!(list[0], 69);
    }

    #[test]
    fn can_index_2() {
        let mut list = SingleLinkedList::<i32>::new();
        list.append(69);
        list.append(666);
        assert_eq!(list[1], 666);
    }

    #[test]
    #[should_panic]
    fn can_index_panic() {
        let mut list = SingleLinkedList::<i32>::new();
        list.append(69);
        list.append(666);
        list[2];
    }

    #[test]
    fn can_iter() {
        let mut list = SingleLinkedList::<i32>::new();
        list.append(69);
        let mut it = list.iter();
        assert_eq!(it.next().unwrap(), &69);
    }

    #[test]
    fn can_iter_2() {
        let mut list = SingleLinkedList::<i32>::new();
        list.append(69);
        list.append(666);
        let mut it = list.iter();
        assert_eq!(it.next().unwrap(), &69);
        assert_eq!(it.next().unwrap(), &666);
    }
}
