use std::ops::Deref;

#[derive(Debug)]
pub struct Node<T>
where
    T: Clone,
{
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}
impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }
}
pub trait LinkedListOps<T>
where
    T: Clone,
{
    fn head(&self) -> Option<&Box<Node<T>>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn push_back(&mut self, data: T);
    fn push_front(&mut self, data: T);
    fn delete_front(&mut self) -> bool;
    fn delete_end(&mut self) -> Option<Box<Node<T>>>;
}

pub struct LinkedList<T>
where
    T: Clone,
{
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        LinkedList {
            head: None,
            len: 0,
        }
    }

    pub fn new_fixed<const CAPACITY: usize>(arr: [T; CAPACITY]) -> Self {
        let mut new_list = LinkedList::new();
        for item in arr {
            new_list.push_back(item);
        }
        new_list
    }

    pub fn new_with(arr: &[T]) -> Self {
        let mut new_list = LinkedList::new();
        for item in arr {
            new_list.push_back(item.clone());
        }
        new_list
    }
}
impl<T> LinkedListOps<T> for LinkedList<T>
where
    T: Clone,
{
    fn head(&self) -> Option<&Box<Node<T>>> {
        self.head.as_ref()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.head.is_none() && self.len == 0
    }

    /// O(1) insert
    fn push_front(&mut self, data: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(data)));
        } else {
            let mut new_node = Box::new(Node::new(data));
            new_node.next = self.head.take();
            self.head = Some(new_node);
        }
    }

    /// O(n) insert, like for it being naive.
    fn push_back(&mut self, data: T) {
        if self.head.is_none() {
            self.len += 1;
            self.head = Some(Box::new(Node::new(data)));
        } else {
            let mut cursor = self.head.as_mut();
            while let Some(node) = cursor {
                if node.next.is_none() {
                    node.next = Some(Box::new(Node::new(data)));
                    if self.len > 1 {
                        self.len -= 1;
                    }
                    break;
                }
                cursor = node.next.as_mut();
            }
        }
    }

    fn delete_front(&mut self) -> bool {
        match self.head.take() {
            None => false,
            Some(_) => true,
        }
    }

    fn delete_end(&mut self) -> Option<Box<Node<T>>> {
        let mut cursor = &mut self.head;
        while let Some(x) = cursor {
            let next_node = x.next.as_ref();
            if next_node.is_some() && next_node.unwrap().next.is_none() {
                let res = x.next.take();
                self.len -= 1;
                return res;
            }
            cursor = &mut x.next;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, LinkedListOps};

    #[test]
    fn test_linked_list_empty() {
        let empty = LinkedList::<i32>::new();
        assert_eq!(empty.head().is_none(), true);
    }
    #[test]
    fn test_linked_list_many_values() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        list.push_back(6);
        assert_eq!(list.head().unwrap().data, 1);
    }
}
