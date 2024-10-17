#[derive(Debug)]
struct Path {
    head: Option<Box<Node>>,
}

impl Path {
    // Creates an empty linked list
    fn new() -> Self {
        Path { head: None }
    }

    // Adds a new element to the front of the list
    fn push(&mut self, value: usize) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(), // Move the current head to the next node
        });
        self.head = Some(new_node); // Set the new node as the head
    }

    // Removes the first element and returns its value
    fn pop(&mut self) -> Option<usize> {
        self.head.take().map(|node| {
            self.head = node.next; // Move the head to the next node
            node.value // Return the value of the popped node
        })
    }

    fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = &node.next; // Move to the next node
        }

        count
    }

    // Peek at the value of the head node
    fn peek(&self) -> Option<&usize> {
        self.head.as_ref().map(|node| &node.value)
    }

    // Check if the list is empty
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn iter(&self) -> LinkedListIter {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }
}
