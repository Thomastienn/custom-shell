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

    pub fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            let Some(next_node) = node.children.get(&ch) else {
                return Vec::new();
            };

            node = next_node;
        }
        if node.is_word {
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
