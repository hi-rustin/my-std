use std::{cell::RefCell, rc::Rc};

/// A node in a singly linked list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T: Clone> {
    elem: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

/// A singly linked list with a reference-counted `Node` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T: Clone> LinkedList<T> {
    /// Create a new, empty `LinkedList`.
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }

    /// Add an element to the front of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_std::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn push(&mut self, elem: T) {
        let new_node = Rc::new(RefCell::new(Node { elem, next: None }));

        match self.head {
            Some(ref head) => {
                let mut current = head.clone();
                while current.borrow().next.is_some() {
                    let next = current.borrow().next.clone().unwrap();
                    current = next;
                }
                current.borrow_mut().next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }

        self.len += 1;
    }

    /// Remove an element from the front of the list.
    /// Returns `None` if the list is empty.
    ///
    /// # Examples
    /// ```
    /// use my_std::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.pop(), Some(1));
    /// assert_eq!(list.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let mut prev = self.head.as_ref().unwrap().clone();
        let mut current = prev.borrow().next.clone();
        for _ in 1..(self.len() - 1) {
            let next = current.as_ref().unwrap().borrow().next.clone();
            prev = current.unwrap();
            current = next;
        }

        if current.is_none() {
            self.head = None;
            self.len -= 1;
            return Some(prev.borrow().elem.clone());
        }

        let res = current.as_ref().unwrap().borrow().elem.clone();
        prev.borrow_mut().next = None;
        self.len -= 1;

        Some(res)
    }

    /// Insert an element at the given index.
    /// Panics if the index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use my_std::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// list.insert(0, 3);
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.pop(), Some(3));
    /// assert_eq!(list.pop(), Some(1));
    /// ```
    pub fn insert(&mut self, index: usize, elem: T) {
        if index >= self.len() {
            panic!(
                "insertion index (is {index}) should be <= len (is {len})",
                len = self.len()
            );
        }

        let mut current = self.head.as_ref().unwrap().clone();
        for _ in 0..index {
            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next;
        }

        let new_node = Rc::new(RefCell::new(Node {
            elem,
            next: current.borrow().next.clone(),
        }));

        current.borrow_mut().next = Some(new_node);
        self.len += 1;
    }

    /// Remove an element at the given index.
    /// Panics if the index is out of bounds.
    /// Returns the removed element.
    ///
    /// # Examples
    /// ```
    /// use my_std::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.remove(1), 2);
    /// assert_eq!(list.remove(0), 1);
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len() {
            panic!(
                "removal index (is {index}) should be < len (is {len})",
                index = index,
                len = self.len()
            );
        }

        let mut current = self.head.as_ref().unwrap().clone();
        let mut prev = None;
        for _ in 0..index {
            prev = Some(current.clone());
            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next;
        }

        let res = current.borrow().elem.clone();
        let next = current.borrow_mut().next.take();
        if let Some(prev) = prev {
            prev.borrow_mut().next = next;
        } else {
            self.head = next;
        }
        self.len -= 1;

        res
    }

    /// Return `true` if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Length of the list.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.insert(0, 3);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn test_remove() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.remove(1), 2);
        assert_eq!(list.remove(0), 1);
    }
}
