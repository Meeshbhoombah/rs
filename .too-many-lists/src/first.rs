use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl Drop for Box<Node> {
   fn drop(&mut self) {
        self.ptr.drop();
        deallocate(self.ptr);
   }
}

impl Drop for Link {
    fn drop(&mut self) {
        match *self {
            Link::Empty => {},
            Link::More(ref mut boxed_node) => {
                boxed_node.drop();
            }
        }
    }
}

pub struct List {
    head: Link,
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

impl List {
    pub fn new() -> Self {
        List { 
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,   
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Nothing results from a pop operation on an empty list
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        
        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
    }
}

