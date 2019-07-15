use std::mem;

pub struct List<T> {
    head: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = link {
            link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {
        let mut list: super::List<i32> = super::List::new();

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
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
