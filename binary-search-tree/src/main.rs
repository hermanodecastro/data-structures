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

       // println!("{:?}\n\n\n", self.root);
    }

    //to do
    fn lookup(key: i32) -> bool {
        true
    }

    fn min(&mut self) -> i32 {
        fn callback(root: &mut Box<Node>) -> i32 {
            match &mut root.left {
                Some(left) => callback(left),
                None => root.value
            }
        }

        match &mut self.root {
            Some(root) => callback(root),
            None => 0
        }
    }

    fn max(&mut self) -> i32 {
        fn callback(root: &mut Box<Node>) -> i32 {
            match &mut root.right {
                Some(right) => callback(right),
                None => root.value
            }
        }

        match &mut self.root {
            Some(root) => callback(root),
            None => 0
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
    
    println!("Min: {}", tree.min());
    println!("Max: {}", tree.max());
}

#[test]
fn test_insert() {
    let mut tree = BinaryTree::new();
    let node = Box::new(Node::new(15));

    tree.insert(Box::new(Node::new(10)));

    if let Some(root) = &tree.root {
        assert_eq!(root.value, node.value);
    } 
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

    assert_eq!(tree.min(), 1);
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

    assert_eq!(tree.max(), 25);
}

