use std::rc::Rc;

pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { head: None }
    }

    pub fn append(&self, elem: T) -> Stack<T> {
        Stack {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Stack<T> {
        Stack {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn basics() {
        let stack = Stack::new();
        assert_eq!(stack.head(), None);

        let stack = stack.append(1).append(2).append(3);
        assert_eq!(stack.head(), Some(&3));

        let stack = stack.tail();
        assert_eq!(stack.head(), Some(&2));

        let stack = stack.tail();
        assert_eq!(stack.head(), Some(&1));

        let stack = stack.tail();
        assert_eq!(stack.head(), None);

        let stack = stack.tail();
        assert_eq!(stack.head(), None);
    }

    #[test]
    fn iter() {
        let stack = Stack::new().append(1).append(2).append(3);
        let mut iter = stack.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
