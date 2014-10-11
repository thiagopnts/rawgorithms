extern crate alloc;

use std::mem;
use self::alloc::arc::Arc;
use std::ptr;


struct RawLink<T> {
    p: *mut T
}

/// RawLink is a type like Option<T> but for holding a raw pointer
impl<T> RawLink<T> {
    fn none() -> RawLink<T> {
        RawLink{p: ptr::null_mut()}
    }

    fn some(n: &mut T) -> RawLink<T> {
        RawLink{p: n}
    }

    fn resolve_mut<'a>(&self) -> Option<&'a T> {
        unsafe {
            self.p.as_ref()
        }
    }

    fn resolve<'a>(&mut self) -> Option<&'a mut T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }

    fn take(&mut self) -> RawLink<T> {
        mem::replace(self, RawLink::none())
    }
}

impl<T> Clone for RawLink<T> {
    #[inline]
    fn clone(&self) -> RawLink<T> {
        RawLink{p: self.p}
    }
}

struct Node<T> {
    next: RawLink<Node<T>>,
    value: T
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node{value: value, next: RawLink::none()}
    }
}

pub struct Stack<T> {
    length: uint,
    last: RawLink<Node<T>>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack{length: 0, last: RawLink::none()}
    }

    pub fn push(&mut self, value: T) {
        self.push_back(box Node::new(value));
    }

    pub fn len(&self) -> uint {
        self.length
    }

    fn push_back(&mut self, mut new_last: Box<Node<T>>) {
        match self.last.resolve() {
            None => {
                self.last = RawLink::some(&mut *new_last);
            },
            Some(last) => {
                self.last = RawLink::some(&mut *new_last);
                //last.next = link_with_prev etc
            }
        }
        self.length += 1
    }

}

