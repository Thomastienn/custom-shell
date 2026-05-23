pub struct Node<T> {
    pub value: T,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

pub struct DoublyLinkedList<T> {
    pub nodes: Vec<Node<T>>,
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_idx = self.nodes.len();

        let new_node = Node {
            value,
            prev: self.tail,
            next: None,
        };

        self.nodes.push(new_node);

        match self.tail {
            Some(old_tail_idx) => {
                self.nodes[old_tail_idx].next = Some(new_idx);
            }
            None => {
                self.head = Some(new_idx);
            }
        }

        self.tail = Some(new_idx);
        self.length += 1;
    }

    pub fn remove_idx(&mut self, idx: usize) {
        if idx >= self.nodes.len() {
            return;
        }

        let prev_idx = self.nodes[idx].prev;
        let next_idx = self.nodes[idx].next;

        if let Some(prev_idx) = prev_idx {
            self.nodes[prev_idx].next = next_idx;
        } else {
            self.head = next_idx;
        }

        if let Some(next_idx) = next_idx {
            self.nodes[next_idx].prev = prev_idx;
        } else {
            self.tail = prev_idx;
        }

        self.length -= 1;
    }
}
