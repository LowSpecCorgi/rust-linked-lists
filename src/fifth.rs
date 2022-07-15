/*
    Linkedlists are really starting to bore me now, but I need to learn how to handle unsafe :()
*/

/*
Layout:

Input: ptr -> (a, ptr) -> (b, None)
Push(x): ptr -> (x, ptr) -> (a, ptr) -> (b, None)
Pop(x): ptr -> (a, ptr) -> (b, None)
*/

use std::mem;

// ;)
use std::ptr;

/// Creates a new `LinkedList`
/// 
/// A tail is added for performance reasons.
/// Why traverse the list to reach the end, when you can store a pointer to it?
pub struct List<T> {
    head: Link<T>,
    /// From my time learning rust, I have been convinced that this is heresy.
    /// Maybe some crab will jump me while sleeping.
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    /// Pushes an element to the tail
    /// 
    /// Replaces the tail with a new tail and then the old tail is updated to reflect that
    pub fn push(& mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            // I hope this causes something bad
            // like a dangling pointer or explosion
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }
    
    /// Takes an element from the head
    /// 
    /// Takes the head and returns it
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            // If there is no head, set the tail to none
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);list.push(2);list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(6); list.push(7);

        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}