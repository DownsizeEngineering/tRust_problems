use std::collections::HashMap;
use std::boxed::Box;

pub struct CharTrie {
    end: bool,
    children: HashMap<char, Box<CharTrieNode>>
}

#[derive(Debug)]
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

    pub fn is_end(&self) -> bool { return self.end; }

    pub fn step(&self, c: char) -> Option<&Box<CharTrieNode>> {
        if let Some(node) = self.children.get(&c) {
            return Some(node);
        } else { return None; }
    }

    fn insert(&mut self, word: &str) {
        if word.len() == 0 { return self.end(); }

        let c: char = word[0..1].parse().unwrap();
        let word = &word[1..];

        if let Some(node) = self.children.get_mut(&c) {
            node.insert(word);
        } else {
            let mut node = CharTrieNode::new(c);
            node.insert(word);
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

    pub fn is_end(&self) -> bool { return self.end; }

    pub fn step(&self, c: char) -> Option<&Box<CharTrieNode>> {
        if let Some(node) = self.children.get(&c) {
            return Some(node);
        } else { return None; }
    }

    pub fn insert(&mut self, word: String) {
        if word.len() == 0 { return self.end(); }

        let c: char = word[0..1].parse().unwrap();
        let word = &word[1..];

        if let Some(node) = self.children.get_mut(&c) {
            node.insert(word);
        } else {
            let mut node = CharTrieNode::new(c);
            node.insert(word);
            self.children.insert(c, Box::new(node));
        }
    }

    pub fn new(words: Vec<String>) -> CharTrie {

        let mut output = CharTrie {
            end: false,
            children: HashMap::new()
        };

        for word in words { output.insert(word); }
        output
    }
}

pub fn run() {
    let trie = CharTrie::new(vec![String::from("banana")]);
    println!("tc {:?}", trie.children);
    let a = trie.step('b').unwrap();
    println!("a {}", a.val);
    let b = a.step('a').unwrap();
    let c = b.step('n');
}