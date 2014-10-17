#[deriving(Clone)]
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
    length: uint,
    first: Option<Box<Node<T>>>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack{length: 0, first: None}
    }

    pub fn push(&mut self, value: T) {
        self.length += 1;
        self.first = Some(box Node {
            data: value,
            next: self.first.take()
        })
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

    pub fn len(&self) -> uint {
        self.length
    }
}

