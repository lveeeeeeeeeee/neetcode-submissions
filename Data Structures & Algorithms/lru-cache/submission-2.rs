use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
struct LRUCacheNode {
    pub next: Option<Rc<RefCell<LRUCacheNode>>>,
    pub prev: Option<Rc<RefCell<LRUCacheNode>>>,
    pub val: i32,
    key: i32
}

impl LRUCacheNode {
    pub fn new(val: i32, key: i32) -> Self {
        LRUCacheNode {
            next: None,
            prev: None,
            val,
            key
        }
    }
}

struct LRUCache {
    __head: Option<Rc<RefCell<LRUCacheNode>>>,
    __tail: Option<Rc<RefCell<LRUCacheNode>>>,
    __kv: HashMap<i32, Option<Rc<RefCell<LRUCacheNode>>>>,
    pub capacity: usize,
    pub count: usize
}

impl LRUCache {
    // why tf is capacity a signed integer
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            __head: None,
            __tail: None,
            __kv: HashMap::with_capacity(capacity as usize),
            capacity: capacity as usize,
            count: 0usize
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let Some(opt) = self.__kv.get(&key) else {
            return -1;
        };
        let node = opt.as_ref().unwrap().clone();
        let value = node.borrow().val;
        self.modify(node, value);
        value
    }

    fn modify(&mut self, node: Rc<RefCell<LRUCacheNode>>, value: i32) {
        // if already tail, just update value
        if let Some(tail) = self.__tail.as_ref() {
            if Rc::ptr_eq(tail, &node) {
                node.borrow_mut().val = value;
                return;
            }
        }
        let node_stitch = Some(node.clone());
        // modify the node itself
        let mut node_ref = node.borrow_mut();
        node_ref.val = value;
        // go stitch
        let next_opt = node_ref.next.take();
        let prev_opt = node_ref.prev.take();
        match (prev_opt, next_opt) {
            // inbetween head and tail
            (Some(prev), Some(next)) => {
                // rc clones
                let next_stitch = Some(next.clone());
                let prev_stitch = Some(prev.clone());
                next.borrow_mut().prev = prev_stitch;
                prev.borrow_mut().next = next_stitch;
            },
            // at head
            (None, Some(next)) => {
                self.__head = Some(next.clone());
                next.borrow_mut().prev = None;
            },
            (Some(_), None) => {},
            _ => {}
        }
        if let Some(old_tail) = self.__tail.as_ref() {
            old_tail.borrow_mut().next = node_stitch.clone();
            node_ref.prev = Some(old_tail.clone());
        }
        self.__tail = node_stitch;
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let mr = &mut self.__kv;
        if self.count == 0 {
            let node = Rc::new(RefCell::new(LRUCacheNode::new(value, key)));
            self.__head = Some(node.clone());
            self.__tail = Some(node.clone());
            mr.insert(key, Some(node.clone()));
            self.count += 1;
        } else if let Some(node_opt) = mr.get(&key) {
            // already in map => modification
            let node = node_opt.as_ref().unwrap().clone();
            print!("modifying {} ", value);
            self.modify(node, value);
        } else {
            if self.count >= self.capacity {
                // no space available => deletion
                let to_delete = self.__head.as_ref().unwrap().clone();
                let to_delete_key = to_delete.borrow().key;
                mr.remove(&to_delete_key);
                if let Some(curr) = to_delete.borrow_mut().next.as_ref() {
                    self.__head = Some(curr.clone());
                    curr.borrow_mut().prev = None;
                } else {
                    self.__head = Some(self.__tail.as_ref().unwrap().clone());
                }
                self.count -= 1;
            }
            println!("adding {}", value);
            // not in map and space is available => addition 
            let mut node = Rc::new(RefCell::new(LRUCacheNode::new(value, key)));
            if self.count == 0 {
                self.__head = Some(node.clone());
                self.__tail = Some(node.clone());
            } else {
                let b_tail = self.__tail.as_ref().unwrap();
                node.borrow_mut().prev = Some(b_tail.clone());
                b_tail.borrow_mut().next = Some(node.clone());
                self.__tail = Some(node.clone());
            }
            mr.insert(key, Some(node));
            self.count += 1;
        }
    }
}