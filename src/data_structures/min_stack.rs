pub struct Stack<T> {
    pub stack: Vec<T>,
    min_stack: Vec<T>,
}

impl<T> Stack<T>
where
    T: Ord + Copy,
{
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn add(&mut self, num: T) {
        self.stack.push(num);
        let new_min = if self.min_stack.is_empty() {
            num
        } else {
            num.min(self.min_stack.last().copied().unwrap())
        };

        self.min_stack.push(new_min);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    pub fn get_min(&self) -> Option<&T> {
        self.min_stack.last()
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn test_min_stack() {
        let mut stack = Stack::new();
        stack.add(1);
        stack.add(2);
        stack.add(3);
        stack.add(4);
        stack.add(5);
        stack.add(6);
        stack.add(7);
        stack.add(8);
        let min = stack.get_min().unwrap();
        assert_eq!(*min, 1);
        stack.pop();
        stack.pop();
        stack.pop();
        let min = stack.get_min().unwrap();
        assert_eq!(*min, 1);
        stack.pop();
        stack.pop();
        stack.pop();
        stack.pop();
        let min = stack.get_min().unwrap();
        assert_eq!(*min, 1);
        stack.pop();
        let min = stack.get_min();
        assert_eq!(min, None);
    }

    #[test]
    fn test_min_stack2() {
        let mut stack = Stack::new();
        stack.add(8);
        stack.add(7);
        stack.add(6);
        let min = stack.get_min().unwrap();
        assert_eq!(*min, 6);
        stack.pop();
        let min = stack.get_min().unwrap();
        assert_eq!(*min, 7);
        stack.pop();
        let min = stack.get_min().unwrap();
        assert_eq!(*min, 8);
    }
}
