use std::mem;

pub struct Stack<T> {
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

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { head: Link::Empty }
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

impl<T> Drop for Stack<T> {
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
        let mut stack: super::Stack<i32> = super::Stack::new();

        assert_eq!(stack.pop(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);
        stack.push(5);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
