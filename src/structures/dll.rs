use std::collections::HashMap;
use std::hash::Hash;

pub trait HasId {
    type Id: Eq + Hash + Clone;

    fn id(&self) -> Self::Id;
}

pub struct Node<T: HasId> {
    pub value: T,
    pub prev: Option<T::Id>,
    pub next: Option<T::Id>,
}

pub struct DoublyLinkedList<T: HasId> {
    pub nodes: HashMap<T::Id, Node<T>>,
    pub head: Option<T::Id>,
    pub tail: Option<T::Id>,
}

impl<T: HasId> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            prev: None,
            next: None,
        }
    }
}

impl<T: HasId> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    pub fn ids(&self) -> Vec<T::Id> {
        let mut ids = Vec::new();
        let mut current = self.head.clone();

        while let Some(id) = current {
            ids.push(id.clone());
            current = self
                .nodes
                .get(&id)
                .and_then(|node| node.next.clone());
        }
        ids
    }

    pub fn _is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn get_node(&self, id: &T::Id) -> Option<&Node<T>> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: &T::Id) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id)
    }

    pub fn push_back(&mut self, value: T) {
        let id = value.id();

        if self.nodes.contains_key(&id) {
            panic!("duplicate node id");
        }

        let mut new_node = Node::new(value);
        new_node.prev = self.tail.clone();

        self.nodes.insert(id.clone(), new_node);

        match self.tail.clone() {
            Some(old_tail_id) => {
                self.nodes
                    .get_mut(&old_tail_id)
                    .unwrap()
                    .next = Some(id.clone());
            }
            None => {
                self.head = Some(id.clone());
            }
        }

        self.tail = Some(id);
    }

    pub fn _push_front(&mut self, value: T) {
        let id = value.id();

        if self.nodes.contains_key(&id) {
            panic!("duplicate node id");
        }

        let mut new_node = Node::new(value);
        new_node.next = self.head.clone();

        self.nodes.insert(id.clone(), new_node);

        match self.head.clone() {
            Some(old_head_id) => {
                self.nodes
                    .get_mut(&old_head_id)
                    .unwrap()
                    .prev = Some(id.clone());
            }
            None => {
                self.tail = Some(id.clone());
            }
        }

        self.head = Some(id);
    }

    pub fn remove(&mut self, id: &T::Id) -> Option<T> {
        let node = self.nodes.remove(id)?;

        match node.prev.clone() {
            Some(prev_id) => {
                self.nodes
                    .get_mut(&prev_id)
                    .unwrap()
                    .next = node.next.clone();
            }
            None => {
                self.head = node.next.clone();
            }
        }

        match node.next.clone() {
            Some(next_id) => {
                self.nodes
                    .get_mut(&next_id)
                    .unwrap()
                    .prev = node.prev.clone();
            }
            None => {
                self.tail = node.prev.clone();
            }
        }

        Some(node.value)
    }

    pub fn _get(&self, id: &T::Id) -> Option<&T> {
        self.nodes.get(id).map(|node| &node.value)
    }

    pub fn _get_mut(&mut self, id: &T::Id) -> Option<&mut T> {
        self.nodes.get_mut(id).map(|node| &mut node.value)
    }

    pub fn _pop_front(&mut self) -> Option<T> {
        let id = self.head.clone()?;
        self.remove(&id)
    }

    pub fn _pop_back(&mut self) -> Option<T> {
        let id = self.tail.clone()?;
        self.remove(&id)
    }
}
