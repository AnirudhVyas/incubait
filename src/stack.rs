use std::fmt::Error;

pub struct Stack<T, const N: usize>
where
    T: Default + Copy,
{
    pub len: usize,
    pub data: [Option<T>; N],
}

impl<T, const N: usize> Stack<T, N>
where
    T: Default + Copy,
{
    pub const fn new() -> Stack<T, N> {
        Self {
            len: 0,
            data: [None; N],
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), Box<dyn std::error::Error>> {
        if self.len > N {
            Err("stack overflow".into())
        } else {
            self.data[self.len] = Some(item);
            self.len += 1;
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let res = self.data[self.len].take();
            self.len -= 1;
            res
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data[self.len - 1].as_ref()
    }
}

#[cfg(test)]
mod stack_tests {
    use crate::stack::Stack;

    #[test]
    pub fn test_push() {
        let mut stack: Stack<usize, 4> = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
    }

    #[test]
    pub fn test_pop() {
        let mut stack: Stack<usize, 4> = Stack::new();
        let mut res = stack.push(1);
        match res {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        assert_eq!(stack.peek(), Some(&1));
        res = stack.push(2);
        match res {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        res = stack.push(3);
        match res {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        assert_eq!(stack.peek(), Some(&3));
        stack.pop();
        assert_eq!(stack.peek(), Some(&2));
    }
}
