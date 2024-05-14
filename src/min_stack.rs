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
