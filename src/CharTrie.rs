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

    pub fn contains(&self, string: &str) -> bool {
        if string.len() == 0 && self.end { return true; }

        let c: char = string[0..1].parse().unwrap();
        let string = &string[1..];

        if let Some(node) = self.children.get(&c) {
            return node.contains(string);
        }

        return false;
    }
}

pub fn search(phrase: &str, dictionary: Vec<String>) -> Vec<Vec<String>> {
    let mut output = Vec::new();
    let dictionary = CharTrie::new(dictionary);

    fn recurse(
        phrase: &str, 
        mut words: Vec<String>,
        output: &mut Vec<Vec<String>>,
        dictionary: &CharTrie) {
        if phrase.is_empty() { return output.push(words); }
        let mut word = String::new();

        let mut chars = phrase.chars();
        let mut step = dictionary.step(chars.next().unwrap());
        let mut steps: usize = 1;

        if let Some(node) = step {
            word.push(node.val);
            if node.is_end() {
                let mut words = words.clone();
                words.push(word.clone());
                let phrase = &phrase[steps..phrase.len()]; 

                recurse(phrase, words, output, dictionary);
            }       
        }
        else { return; }

        for char in chars {
            if let Some(current) = step {
                step = current.step(char);
                steps += 1;

            }
            if let Some(node) = step {
                word.push(node.val);
                if node.is_end() {
                    let mut words = words.clone();
                    words.push(word.clone());
                    let phrase = &phrase[steps..phrase.len()]; 

                    recurse(phrase, words, output, dictionary);
                }
            }

        }     
        

    }

    recurse(phrase, Vec::new(), &mut output, &dictionary);
    output
}

pub fn run() {
    // let trie = CharTrie::new(vec![String::from("banana"), String::from("ban")]);
    // let mut a = trie.step('b').unwrap();
    // println!("a {}", a.val);
    // a = a.step('a').unwrap();
    // a = a.step('n').unwrap();
    // println!("c: {:?}", a);
    // println!("tc {:?}", trie.children);

    let words = vec![String::from("re"), String::from("dc"), String::from("at"),
     String::from("red"), String::from("cat"), String::from("a"), 
     String::from("r"), String::from("edca")];
    let phrase = "redcat";

    // let trie = CharTrie::new(words);
    // println!("{}", trie.contains("re"))

    println!("{:?}", search(phrase, words));
    // println!("{}", &phrase[phrase.len()-1..phrase.len()])
}