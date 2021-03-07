use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
    value: i32,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            right: None,
            left: None,
        }
    }
}

#[derive(Debug)]
struct BinaryTree {
    root: Option<Box<Node>>
}

impl BinaryTree {
    fn new() -> Self {
        Self {
            root: None
        }
    }

    fn insert(&mut self, node: Box<Node>) {
        fn callback(root: &mut Box<Node>, node: Box<Node>) {
            if node.value > root.value {
                match &mut root.right {
                    Some(right) => callback(right, node),
                    None => root.right = Some(node)
                }
            } else {
                match &mut root.left {
                    Some(left) => callback(left, node),
                    None => root.left = Some(node)
                }
            }
        }

        match &mut self.root {
            Some(root) => callback(root, node),
            None => self.root = Some(node)
        }
    }

    fn lookup(&mut self, key: i32) -> bool {
        fn callback(key: i32, root: &mut Box<Node>) -> bool {
            match key.cmp(&root.value) {
                Ordering::Less => match &mut root.left {
                    Some(left) => callback(key, left),
                    None => false
                },
                Ordering::Greater => match &mut root.right {
                    Some(right) => callback(key, right),
                    None => false
                },
                Ordering::Equal => true
            }
        }

        match &mut self.root {
            Some(root) => callback(key, root),
            None => false
        }
    }

    fn min(&mut self) -> Option<i32> {
        fn callback(root: &mut Box<Node>) -> Option<i32> {
            match &mut root.left {
                Some(left) => callback(left),
                None => Some(root.value)
            }
        }

        match &mut self.root {
            Some(root) => callback(root),
            None => None
        }
    }

    fn max(&mut self) -> Option<i32> {
        fn callback(root: &mut Box<Node>) -> Option<i32> {
            match &mut root.right {
                Some(right) => callback(right),
                None => Some(root.value)
            }
        }

        match &mut self.root {
            Some(root) => callback(root),
            None => None
        }
    }

    fn summary(&mut self) {
        if let Some(min) = &mut self.min() {
            println!("Min: {}", min);
        } else {
            println!("Min: Tree is empty");
        }

        if let Some(max) = &mut self.max() {
            println!("Max: {}", max);
        } else {
            println!("Max: Tree is empty");
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(Box::new(Node::new(5)));
    tree.insert(Box::new(Node::new(15)));
    tree.insert(Box::new(Node::new(25)));
    tree.insert(Box::new(Node::new(3)));
    tree.insert(Box::new(Node::new(20)));
    tree.insert(Box::new(Node::new(1)));
    
    tree.summary();
}

#[test]
fn test_insert() {
    let mut tree = BinaryTree::new();
    let node = Box::new(Node::new(15));

    tree.insert(Box::new(Node::new(15)));

    if let Some(root) = &tree.root {
        assert_eq!(root.value, node.value);
    } 
}

#[test]
fn test_lookup_should_return_true() {
    let mut tree = BinaryTree::new();

    tree.insert(Box::new(Node::new(5)));
    tree.insert(Box::new(Node::new(15)));
    tree.insert(Box::new(Node::new(25)));
    tree.insert(Box::new(Node::new(3)));
    tree.insert(Box::new(Node::new(20)));
    tree.insert(Box::new(Node::new(1)));

    assert!(tree.lookup(25));
}

#[test]
fn test_lookup_should_return_false() {
    let mut tree = BinaryTree::new();

    tree.insert(Box::new(Node::new(5)));
    tree.insert(Box::new(Node::new(15)));
    tree.insert(Box::new(Node::new(25)));
    tree.insert(Box::new(Node::new(3)));
    tree.insert(Box::new(Node::new(20)));
    tree.insert(Box::new(Node::new(1)));

    assert_eq!(tree.lookup(100), false);
}

#[test]
fn test_min() {
    let mut tree = BinaryTree::new();

    tree.insert(Box::new(Node::new(5)));
    tree.insert(Box::new(Node::new(15)));
    tree.insert(Box::new(Node::new(25)));
    tree.insert(Box::new(Node::new(3)));
    tree.insert(Box::new(Node::new(20)));
    tree.insert(Box::new(Node::new(1)));

    assert_eq!(tree.min(), Some(1));
}

#[test]
fn test_max() {
    let mut tree = BinaryTree::new();

    tree.insert(Box::new(Node::new(5)));
    tree.insert(Box::new(Node::new(15)));
    tree.insert(Box::new(Node::new(25)));
    tree.insert(Box::new(Node::new(3)));
    tree.insert(Box::new(Node::new(20)));
    tree.insert(Box::new(Node::new(1)));

    assert_eq!(tree.max(), Some(25));
}

