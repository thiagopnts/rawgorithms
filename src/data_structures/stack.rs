#[derive(Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node{data: value, next: None}
    }
}

pub struct Stack<T> {
    length: usize,
    first: Option<Box<Node<T>>>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack{length: 0, first: None}
    }

    pub fn push(&mut self, value: T) {
        self.length += 1;
        self.first = Some(Box::new(Node {
            data: value,
            next: self.first.take()
        }));
    }

    pub fn peek(&mut self) -> Option<&T> {
        match self.first {
            None => None,
            Some(ref head) => Some(&head.data)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.first.take() {
            None => None,
            Some(mut head) => {
                self.first = head.next.take();
                self.length -= 1;
                Some(head.data)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(3, stack.len());
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(3, stack.len());

        let last = stack.pop().unwrap();
        assert_eq!(3, last);
        assert_eq!(2, stack.len());

        let last = stack.pop().unwrap();
        assert_eq!(2, last);

        assert_eq!(1, stack.len());
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.push(3);
        stack.push(10);
        stack.push(1);
        assert_eq!(*stack.peek().unwrap(), 1);
        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack: Stack<String> = Stack::new();
        assert_eq!(stack.pop(), None);
    }
}

