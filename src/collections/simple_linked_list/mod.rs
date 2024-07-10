use alloc::{boxed::Box, vec::Vec};
use core::fmt::{self, Debug, Formatter};
use core::ptr::NonNull;

pub struct SimpleLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub const fn new() -> Self {
        SimpleLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, element: T) {
        let node = Box::new(Node {
            element,
            next: self.head,
        });

        let node = Some(Box::leak(node).into());

        self.head = node;

        if self.tail.is_none() {
            self.tail = node;
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, element: T) {
        let node = Box::new(Node {
            element,
            next: None,
        });

        let node = Some(Box::leak(node).into());

        if let Some(tail) = self.tail {
            unsafe {
                (*tail.as_ptr()).next = node;
            }
        } else {
            self.head = node;
        }

        self.tail = node;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            if self.head.is_none() {
                self.tail = None;
            }

            self.len -= 1;
            node.element
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for SimpleLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T: Debug> Debug for SimpleLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut vec = Vec::new();
        let mut current = self.head;
        while let Some(node) = current {
            unsafe {
                vec.push(&(*node.as_ptr()).element);
                current = (*node.as_ptr()).next;
            }
        }
        f.debug_struct("SimpleLinkedList")
            .field("head", &self.head)
            .field("tail", &self.tail)
            .field("len", &self.len)
            .field("elements", &vec)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleLinkedList;

    #[test]
    fn test_new() {
        let list: SimpleLinkedList<i32> = SimpleLinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_front() {
        let mut list = SimpleLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_push_back() {
        let mut list = SimpleLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_pop_front() {
        let mut list = SimpleLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_is_empty() {
        let mut list = SimpleLinkedList::new();
        assert!(list.is_empty());
        list.push_front(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_len() {
        let mut list = SimpleLinkedList::new();
        assert_eq!(list.len(), 0);
        list.push_front(1);
        assert_eq!(list.len(), 1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
        list.pop_front();
        assert_eq!(list.len(), 1);
        list.pop_front();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_front_back() {
        let mut list = SimpleLinkedList::new();
        list.push_front(1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
    }

    #[test]
    fn test_drop() {
        let mut list = SimpleLinkedList::new();
        list.push_front(1);
        list.push_back(2);
        drop(list); // Ensure no memory leaks
    }
}
