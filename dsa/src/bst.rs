pub struct BinarySearchTree<T> 
where
    T: Ord,
{
    value: Option<T>,
    count: i32,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>
}

impl<T> BinarySearchTree<T> 
where
    T: Ord,
{
    pub fn insert(&mut self, value: T) {
        match &self.value {
            None => self.value = Some(value),
            Some(key) => {
                let target_node = if value < *key {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match target_node {
                    Some(ref mut node) => {
                        node.insert(value);
                    }
                    None => {
                        let mut node = new();
                        node.value = Some(value);
                        *target_node = Some(Box::new(node));
                    }
                }
            }
        }
    }

    /*
    pub fn insert2(&mut self, x: T) {
        match &self.value {
            None => {
                self.value = Some(x);
                self.count += 1;
            }
            Some(comparable) => {
                if x < *comparable {
                    let left = &mut self.left;
                    match left {
                        Some(ref mut node) => {
                            node.insert(x);
                        }
                        None => {
                            let mut node = new();
                            node.value = Some(x);
                            *left = Some(Box::new(node));
                        }
                    }
                }
            }
        }
    }
    */

    /*
    pub fn insert(&mut self, x: T) {
        if self.value.is_none() {
            self.value = Some(x); 
            self.count += 1;
        }

        let comparable = self.value.unwrap();
        if x < comparable {
            if self.left.is_none() {
                self.left = Some(Box::new(new()));
            }

            let left = &mut self.left;
            left.insert(x);
        } 

        if x == comparable {
            self.count += 1;
        }

        if x > comparable {
            self.right.insert(x);
        }
    }
    */
}

pub fn new<T>() -> BinarySearchTree<T>
where
    T: Ord 
{
    BinarySearchTree {
        value: None,
        count: 0,
        left: None,
        right: None
    } 
}

