use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn clear(&mut self) {
        self.root = TrieNode::new();
    }

    pub fn insert(&mut self, s: &str) {
        let mut node = &mut self.root;
        for ch in s.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_word = true;
    }

    pub fn _search(&self, s: &str) -> bool {
        let mut node = &self.root;
        for ch in s.chars() {
            let Some(next_node) = node.children.get(&ch) else {
                return false;
            };
            node = next_node;
        }
        node.is_word
    }

    pub fn _starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            let Some(next_node) = node.children.get(&ch) else {
                return false;
            };
            node = next_node;
        }
        true
    }

    fn single_path(&self, node: &TrieNode) -> Option<String> {
        if node.children.is_empty() {
            return Some(String::new());
        }
        if node.children.len() == 1 {
            let (_, next_node) = node.children.iter().next().unwrap();
            let child_path = self.single_path(next_node);
            let mut result = String::new();
            result.push(*node.children.keys().next().unwrap());
            if let Some(child_path) = child_path {
                result.push_str(&child_path);
            } else {
                return None;
            }
            Some(result)
        } else {
            None
        }
    }

    pub fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            let Some(next_node) = node.children.get(&ch) else {
                return Vec::new();
            };

            node = next_node;
        }
        if node.is_word {
            let single_path = self.single_path(node);
            if let Some(full_str) = single_path {
                return vec![format!("{}{}", prefix, full_str)];
            }
            return Vec::new();
        }
        let mut results = Vec::new();
        let mut current = prefix.to_string();

        Self::collect_words(node, &mut current, &mut results);

        results
    }

    fn collect_words(node: &TrieNode, current: &mut String, results: &mut Vec<String>) {
        if node.is_word {
            results.push(current.clone());
        }
        for (&ch, next_node) in &node.children {
            current.push(ch);
            Self::collect_words(next_node, current, results);
            current.pop();
        }
    }
}

pub type CompletionTrie = HashMap<String, Trie>;
