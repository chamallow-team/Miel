use radix_trie::TrieCommon;

#[derive(Debug, Clone)]
pub struct LanguageTree {
    tree: radix_trie::Trie<String, Translation>,
}

impl LanguageTree {
    pub fn new() -> Self {
        Self {
            tree: radix_trie::Trie::new(),
        }
    }

    pub fn get_keys(&self) -> Vec<&String> {
        self.tree.keys().collect::<Vec<&String>>()
    }

    pub fn get_traduction(&self, key: String) -> Option<&Translation> {
        self.tree.get(&key)
    }

    pub fn set_traduction(&mut self, key: String, value: Translation) {
        self.tree.insert(key, value);
    }

    pub fn remove_traduction(&mut self, key: String) {
        self.tree.remove(&key);
    }
}

#[derive(Debug, Clone)]
pub struct Translation {
    pub content: String,
}

impl Translation {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}
