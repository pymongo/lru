use crate::common::prelude::*;

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

struct DoublyLinkedList<K, V> {
    head: ListNode<K, V>,
    tail: ListNode<K, V>
}

struct LruV1<K, V> {
    // TODO limit linkedlist capacity
    // capacity: usize
    head: ListNode<K, V>,
    tail: ListNode<K, V>,
    cache: HashMap<K, *const ListNode<K, V>>
}

impl<K, V> Lru<K, V> for LruV1<K, V> {
    fn new(capacity: usize) -> Self {
        todo!()
    }

    fn put(&mut self, key: K, val: V) {
        todo!()
    }

    fn get(&mut self, key: K) -> Option<V> {
        todo!()
    }
}
