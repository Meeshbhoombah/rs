#[derive(Debug)]
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
    pub fn insert(&mut self, x: T) {
        if self.value.is_none() {
            self.value = Some(x);
            self.count += 1;
            return;
        }

        let comparable = self.value.as_ref().unwrap();
        if x < *comparable {
            if self.left.is_none() {
                self.left = Some(Box::new(new()));
            }
            self.left.as_mut().unwrap().insert(x);
        } else if x == *comparable {
            self.count += 1;
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(new()));
            }
            self.right.as_mut().unwrap().insert(x);
        }
    }
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

