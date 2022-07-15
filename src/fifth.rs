/*
    Linkedlists are really starting to bore me now, but I need to learn how to handle unsafe :()

    No Iter implementation as I genuinely couldn't give a fuck
*/

/*
Layout:

Input: ptr -> (a, ptr) -> (b, None)
Push(x): ptr -> (x, ptr) -> (a, ptr) -> (b, None)
Pop(x): ptr -> (a, ptr) -> (b, None)
*/

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
    tail:  Link<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> List<T> {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    /// Pushes an element to the tail
    /// 
    /// Replaces the tail with a new tail and then the old tail is updated to reflect that
    pub fn push(& mut self, elem: T) {
        // I hope this causes something bad
        // like a dangling pointer or explosion
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem: elem,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }
    
    /// Takes an element from the head
    /// 
    /// Takes the head and returns it
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
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