#[derive(Debug)]
struct Node {
    value: usize,
    next: Option<Box<Node>>,
}

// Iterator struct that holds a reference to the current node
struct LinkedListIter<'a> {
    current: Option<&'a Node>,
}

// Implement the Iterator trait for LinkedListIter
impl<'a> Iterator for LinkedListIter<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If there's a current node, return the value and move to the next node
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}
