#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Black
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    color: Color,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    parent: Option<*mut Node<T>>
}

#[derive(Debug)]
struct RedBlackTree<T> {
    root: Option<Box<Node<T>>>
}

impl<T: Ord> RedBlackTree<T> {
    pub fn insert(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            color: Color::Red,
            left: None,
            right: None,
            parent: None
        });

        if self.root.is_none() {
            self.root = Some(new_node);
            self.root.as_mut().unwrap().color = Color::Black;
        } else {
            self.insert_node(self.root.as_mut().unwrap(), new_node);
        }
    }

    fn insert_node(&mut self, current: &mut Box<Node<T>>, new_node: Box<Node<T>>) {
        if new_node.value < current.value {
            if current.left.is_none() {
                current.left = Some(new_node);
                current.left.as_mut().unwrap().parent = Some(current.as_mut() as *mut Node<T>);
                self.fix_insert(current.left.as_mut().unwrap());
            } else {
                self.insert_node(current.left.as_mut().unwrap(), new_node)
            }
        } else {
            if current.right.is_none() {
                current.right = Some(new_node);
                current.right.as_mut().unwrap().parent = Some(current.as_mut() as *mut Node<T>);
                self.fix_insert(current.right.as_mut().unwrap());
            } else {
                self.insert_node(current.right.as_mut().unwrap(), new_node);
            }
        }
    }

    fn fix_insert(&mut self, node: &mut Box<Node<T>>) {
        while let Some(parent) = unsafe { node.parent.as_ref().map(|p| &**p) } {
            if parent.color == Color::Black {
                break;
            }

            let grandparent = unsafe { parent.parent.unwrap().as_ref().unwrap() };

            if parent == grandparent.left.unwrap().as_ref() {
                let uncle = grandparent.right.as_ref();

                if let Some(uncle) = uncle {
                    if uncle.color == Color::Red {
                        parent.color = Color::Black;
                        uncle.color = Color::Black;
                        grandparent.color = Color::Red;
                        continue;
                    }
                }

                if node == parent.right.unwrap().as_ref() {
                    self.left_rotate(parent);
                    node = parent.left.unwrap().as_mut().unwrap();
                }

                parent.color = Color::Black;
                grandparent.color = Color::Red;
                self.right_rotate(grandparent);
            } else {
                let uncle = grandparent.left.as_ref();

                if let Some(uncle) = uncle {
                    if uncle.color == Color::Red {
                        parent.color = Color::Black;
                        uncle.color = Color::Black;
                        grandparent.color = Color::Red;
                        continue;
                    }
                }

                if node == parent.left.unwrap().as_ref() {
                    self.right_rotate(parent);
                    node = parent.right.unwrap().as_mut().unwrap();
                }

                parent.color = Color::Black;
                grandparent.color = Color::Red;
                self.left_rotate(grandparent);
            }
        }

        self.root.as_mut().unwrap().color = Color::Black;
    }

    fn left_rotate(&mut self, node: &mut Box<Node<T>>)  {
        let right_child = node.right.take();
        node.right = right_child.unwrap().left;
        if let Some(right_left) = &mut node.right {
            right_left.praent = Some(node.as_mut() as *mut Node<T>);
        }
    }
}


pub fn new<T: Ord>() -> RedBlackTree<T> {
    RedBlackTree {
        root: None
    }
}

