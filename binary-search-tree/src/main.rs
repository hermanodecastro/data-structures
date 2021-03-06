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

            return 
        }

        match &mut self.root {
            Some(root) if node.value > root.value => match &mut root.right {
                Some(right) => callback(right, node),
                None => root.right = Some(node)
            },
            Some(root) if node.value < root.value => match &mut root.left {
                Some(left) => callback(left, node),
                None => root.left = Some(node)
            },
            None => self.root = Some(node),
            _ => return
        }

        println!("{:?}\n\n\n", self.root);
    }

    //to do
    fn lookup(key: i32) -> bool {

    }
}

fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(Box::new(Node::new(5)));
    tree.insert(Box::new(Node::new(15)));
    tree.insert(Box::new(Node::new(25)));
    tree.insert(Box::new(Node::new(3)));

    /*
                        5
                      /   \
                     3    15
                            \
                             25
    */
}

