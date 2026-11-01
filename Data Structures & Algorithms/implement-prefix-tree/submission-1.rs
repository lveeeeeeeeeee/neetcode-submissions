struct PrefixNode {
    // if wrapped in option the alignment would be worse iirc
    next: [Option<Box<PrefixNode>>; 26],
    
    // need to store whether we inserted a word or only an intermediate node in the tree
    is_word: bool
}

impl PrefixNode {
    pub fn new() -> Self {
        Self {
            next: Default::default(),
            is_word: false
        }
    }
}

struct PrefixTree {
    root: PrefixNode
}

impl PrefixTree {
    pub fn new() -> Self {
        Self {
            root: PrefixNode::new()
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        for c in word.bytes() {
            let chr = (c - b'a') as usize;
            cur = cur.next[chr].get_or_insert_with(|| Box::new(PrefixNode::new()));
        }
        cur.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut cur = &self.root;
        for c in word.bytes() {
            let chr = (c - b'a') as usize;
            match &cur.next[chr] {
                Some(child) => cur = child,
                None => return false
            }
        }
        cur.is_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;
        for c in prefix.bytes() {
            let chr = (c - b'a') as usize;
            match &cur.next[chr] {
                Some(child) => cur = child,
                None => return false
            }
        }
        true
    }
}
