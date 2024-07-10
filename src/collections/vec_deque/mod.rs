#![no_std]
extern crate alloc;

use alloc::vec::Vec;

pub struct VecDeque<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> VecDeque<T> {
    pub fn new() -> Self {
        VecDeque {
            buffer: Vec::new(),
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize_with(capacity, || None);
        VecDeque {
            buffer,
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        if self.size == self.buffer.len() {
            self.grow();
        }
        self.buffer[self.tail] = Some(value);
        self.tail = (self.tail + 1) % self.buffer.len();
        self.size += 1;
    }

    pub fn push_front(&mut self, value: T) {
        if self.size == self.buffer.len() {
            self.grow();
        }
        self.head = (self.head + self.buffer.len() - 1) % self.buffer.len();
        self.buffer[self.head] = Some(value);
        self.size += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.tail = (self.tail + self.buffer.len() - 1) % self.buffer.len();
        let value = self.buffer[self.tail].take();
        self.size -= 1;
        value
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let value = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.buffer.len();
        self.size -= 1;
        value
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.buffer.len()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn front(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            self.buffer[self.head].as_ref()
        }
    }

    pub fn back(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            let tail = (self.tail + self.buffer.len() - 1) % self.buffer.len();
            self.buffer[tail].as_ref()
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            deque: self,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            deque: self,
            index: 0,
        }
    }

    fn grow(&mut self) {
        let new_capacity = (self.buffer.len().max(1)) * 2;
        let mut new_buffer = Vec::with_capacity(new_capacity);
        new_buffer.resize_with(new_capacity, || None);

        for i in 0..self.size {
            let idx = (self.head + i) % self.buffer.len();
            new_buffer[i] = self.buffer[idx].take();
        }

        self.head = 0;
        self.tail = self.size;
        self.buffer = new_buffer;
    }
}

pub struct Iter<'a, T> {
    deque: &'a VecDeque<T>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.deque.size {
            None
        } else {
            let idx = (self.deque.head + self.index) % self.deque.buffer.len();
            self.index += 1;
            self.deque.buffer[idx].as_ref()
        }
    }
}

pub struct IterMut<'a, T> {
    deque: &'a mut VecDeque<T>,
    index: usize,
}

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index >= self.deque.size {
//             None
//         } else {
//             let idx = (self.deque.head + self.index) % self.deque.buffer.len();
//             self.index += 1;
//             self.deque.buffer[idx].as_mut()
//         }
//     }
// }

impl<T> Drop for VecDeque<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::VecDeque;

    #[test]
    fn test_new() {
        let deque: VecDeque<i32> = VecDeque::new();
        assert_eq!(deque.len(), 0);
        assert!(deque.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let deque: VecDeque<i32> = VecDeque::with_capacity(10);
        assert_eq!(deque.len(), 0);
        assert!(deque.is_empty());
        assert!(!deque.is_full());
    }

    #[test]
    fn test_push_back() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.back(), Some(&2));
    }

    #[test]
    fn test_push_front() {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        deque.push_front(2);
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.front(), Some(&2));
    }

    #[test]
    fn test_pop_back() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn test_pop_front() {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        deque.push_front(2);
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn test_clear() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.clear();
        assert_eq!(deque.len(), 0);
        assert!(deque.is_empty());
    }

    #[test]
    fn test_front() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        assert_eq!(deque.front(), Some(&1));
    }

    #[test]
    fn test_back() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        assert_eq!(deque.back(), Some(&2));
    }

    #[test]
    fn test_iter() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        let mut iter = deque.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    // #[test]
    // fn test_iter_mut() {
    //     let mut deque = VecDeque::new();
    //     deque.push_back(1);
    //     deque.push_back(2);
    //     deque.push_back(3);
    //     let mut iter = deque.iter_mut();
    //     assert_eq!(iter.next(), Some(&mut 1));
    //     assert_eq!(iter.next(), Some(&mut 2));
    //     assert_eq!(iter.next(), Some(&mut 3));
    //     assert_eq!(iter.next(), None);
    // }

    #[test]
    fn test_grow() {
        let mut deque = VecDeque::with_capacity(2);
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&3));
    }
}
