
type TreeNode = Option<Box<Node>>;

struct Node {
pub    left: TreeNode,
pub    right: TreeNode,
pub    value: i32,
}


impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            left: None,
            right: None,
            value: value,
        }
    }

    pub fn insert(&mut self, value: i32) {
        if self.value < value {
            if let Some(ref mut right) = self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node::new(value)));
            }
        }
    }

    pub fn pos_order(&self) {
        if let Some(ref left) = self.left {
            left.pos_order();
        }
        if let Some(ref right) = self.right {
            right.pos_order();
        }
        println!("{}", self.value);
    }

    pub fn pre_order(&self) {
        println!("{}", self.value);

        if let Some(ref left) = self.left {
            left.pre_order();
        }
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    pub fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        println!("{}", self.value);
        if let Some(ref right) = self.right {
            right.in_order();
        }
    }
}

pub type BinaryTree = Node;

#[test]
fn test_insert() {
    let mut root = BinaryTree::new(3);
    root.insert(2);
    root.insert(4);
    root.insert(5);
    root.insert(6);
    root.insert(1);
    if let Some(ref left) = root.left {
        assert_eq!(left.value, 2);
    }

    if let Some(ref right) = root.right {
        assert_eq!(right.value, 4);
        if let Some(ref right) = right.right {
            assert_eq!(right.value, 5);
        }
    }

    println!("In Order traversal");
    root.in_order();
    println!("\n");
    println!("Pos Order traversal");
    root.pos_order();
    println!("Pre Order traversal");
    root.pre_order();
}

