

type Color = Typle3<u8, u8, u8>;

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    data: T
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data: data, left: None, right: None }
    }
}

pub struct KDTree<T> {
    current_i: usize,
    dimensions: usize,
    root: Option<Box<Node<T>>>
}

impl<T> KDTree<T> {
    pub fn new(dimensions: usize) -> KDTree<T> {
        KDTree { current_i: 0, dimensions: dimensions, root: None }
    }

    pub fn insert(&mut self, data: T) {
        self.current_i = 0;
        match self.root.take() {
            None => self.root = Node::new(data),
            Some(mut root) => self.insert_node(root, data)
        }
    }

    fn insert_node(&mut self, node: Box<Node<T>>, data: T) {
    }

    fn index(&mut self) -> usize {
        self.current_i % self.dimensions
    }
}



