use std::fmt::Debug;

use crate::common::prelude::*;

// #[derive(Debug)]
struct ListNode<K, V> {
    key: K,
    val: V,
    next: *mut ListNode<K, V>,
    prev: *mut ListNode<K, V>
}

impl<K: Default, V: Default> Default for ListNode<K, V> {
    fn default() -> Self {
        Self {
            key: K::default(),
            val: V::default(),
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut()
        }
    }
}

struct DoublyLinkedList<K: Default, V: Default> {
    head: ListNode<K, V>,
    tail: ListNode<K, V>
}

struct LruImpl<K: Default, V: Default> {
    // TODO limit linkedlist capacity
    // capacity: usize
    head: ListNode<K, V>,
    tail: ListNode<K, V>,
    cache: HashMap<K, *mut ListNode<K, V>>
}

impl<K: Default, V: Default> LruImpl<K, V> {
    unsafe fn detach_node(&mut self, node: *mut ListNode<K, V>) {
        // detach node
        (*(*node).next).prev = (*(*node).prev).next;
        (*(*node).prev).next = (*(*node).next).next;
    }

    /// before: head->...->node->...->tail
    /// after : head->node->...->tail
    unsafe fn move_node_to_head(&mut self, node: *mut ListNode<K, V>) {
        // let a = (*(*node).prev).next;

        // detach node
        if !(*node).prev.is_null() && !(*node).next.is_null() {
            self.detach_node(node);
        }
        
        // head->next connect to node
        (*node).next = (*self.head.next).prev;
        (*self.head.next).prev = node;
        // head connect to node
        self.head.next = node;
        (*node).prev = &mut self.head;
    }

    // used in debug
    // unsafe fn first()

    // debug impl
    // unsafe fn nomalize() {}
}

impl<K: Default + Hash + Eq + Clone, V: Default + Clone> Lru<K, V> for LruImpl<K, V> {
    fn new(_capacity: usize) -> Self {
        let mut head = ListNode::default();
        let mut tail = ListNode::default();
        head.next = &mut tail;
        tail.prev = &mut head;
        Self {
            head,
            tail,
            cache: HashMap::new()
        }
    }

    unsafe fn put(&mut self, key: K, val: V) {
        if let Some(node_ptr) = self.cache.get(&key) {
            (*(*node_ptr)).val = val;
            self.move_node_to_head(*node_ptr);
            return;
        }

        let mut node = ListNode {
            key,
            val,
            next: self.head.prev,
            prev: &mut self.head,
        };
        self.cache.insert(node.key.clone(), &mut node);
    }

    // FIXME SIGSEGV
    unsafe fn get(&mut self, key: K) -> Option<V> {
        // ownership problem
        let self_ = self as *mut Self;
        (*self_).cache.get(&key).map(|node_ptr| {
            self.move_node_to_head(*node_ptr);
            (*(*node_ptr)).val.clone()
        })
    }
}

#[test]
fn test() {
    let mut lru = LruImpl::<&'static str, &'static str>::new(10);
    unsafe {
        lru.put("key1", "val1");
        lru.put("key2", "val2");
        // dbg!(lru.get("key1"));
    }
}
