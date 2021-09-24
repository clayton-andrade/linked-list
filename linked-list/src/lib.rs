type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T, next: Link<T>) -> Self {
        Node {
            elem,
            next,
        }
    }
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node::new(elem, self.head.take());
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = List::new();
        list.push(10);
        list.push(1024);
        assert_eq!(list.pop(), Some(1024));
        assert_eq!(list.peek(), Some(&10));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.peek(), None);
        assert_eq!(list.pop(), None);
    }
}
