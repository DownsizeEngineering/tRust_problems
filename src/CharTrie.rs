use std::collections::HashMap;
use std::boxed::Box;

pub struct CharTrie {
    end: bool,
    children: HashMap<char, Box<CharTrieNode>>
}

pub struct CharTrieNode {
    val: char,
    end: bool,
    children: HashMap<char, Box<CharTrieNode>>
}

impl CharTrieNode {
    fn new(val: char) -> Self {
        CharTrieNode {
            val,
            end: false,
            children: HashMap::new()
        }
    }

    fn end(&mut self) {
        self.end = true;
    }

    fn insert(&mut self, string: &str) {
        if string.len() == 0 { return self.end(); }

        let c: char = string[0..1].parse().unwrap();
        let string = &string[1..];

        if let Some(node) = self.children.get_mut(&c) {
            node.insert(string);
        } else {
            let mut node = CharTrieNode::new(c);
            node.insert(string);
            self.children.insert(c, Box::new(node));
        }
    }

    fn contains(&self, string: &str) -> bool {
        if string.len() == 0 && self.end { return true; }

        let c: char = string[0..1].parse().unwrap();
        let string = &string[1..];

        if let Some(node) = self.children.get(&c) {
            return node.contains(string);
        }

        return false;
    }
}

impl CharTrie {
    fn end(&mut self) {
        self.end = true;
    }

    pub fn step(&self, c: char) -> Option<Box<CharTrieNode>> {
        None
    }

    pub fn new(words: Vec<String>) -> CharTrie {

        let mut output = CharTrie {
            end: false,
            children: HashMap::new()
        };

        output
    }
}