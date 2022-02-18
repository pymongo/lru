#![allow(dead_code, unused_variables)]
#![feature(linked_list_remove)]

use std::collections::{HashMap, LinkedList};

#[derive(Debug)]
struct Entry {
    k: String,
    v: String
}
#[derive(Debug)]
struct Lru {
    list: LinkedList<Entry>,
    // dummy head and dummy tail

    // 缓存的 value 应该用 链表节点的指针地址，现在用链表索引是因为标准库的链表 remove 只能通过索引
    // 应该 usize 索引村一份，指针地址也村一份
    cache: HashMap<String, (usize, *const Entry)>
}

impl Lru {
    const KEY_DUMMY_HEAD: &'static str = "head";
    #[cfg(FALSE)]
    fn new() -> Self {
        let mut list = LinkedList::<Entry>::new();
        let mut dummy_head = Entry {
            k: Self::KEY_DUMMY_HEAD.to_string(),
            v: "".to_string()
        };
        let dummy_head_ptr = &mut dummy_head as *mut Entry;
        list.push_back(dummy_head);
        let mut cache = HashMap::new();
        cache.insert(Self::KEY_DUMMY_HEAD.to_string(), dummy_head_ptr);
        Self {
            list,
            cache
        }
    }

    /// check entry whether exist
    fn set(&mut self, entry: Entry) {
        if self.cache.contains_key(&entry.k) {
            // 需要将旧数据的节点从链表原有位置剥离出来放到 dummyhead 后面

            // WIP handle key not found
            let old_node = self.cache.remove(&entry.k).unwrap();
            self.list.remove(old_node.0);
        }
        let entry_ptr = &entry as *const Entry;
        self.cache.insert(entry.k.clone(), (0, entry_ptr));
        self.list.push_front(entry);
    }

    fn get(&mut self, k: &str) -> String {
        // WIP handle key not found
        // let entry = self.cache.get(k).unwrap().1;
        // let ret = unsafe { &*entry }.v.clone();

        // TODO 将 entry 链表节点放在靠前位置
        // TODO reuse code
        let old_node = self.cache.remove(k).unwrap();
        let entry = self.list.remove(old_node.0);
        let ret = entry.v.clone();
        self.cache.insert(k.to_string(), (0, &entry as *const Entry));
        self.list.push_front(entry);

        ret
    }
}

#[test]
fn test_lru() {
    let mut  lru = Lru {
        list: LinkedList::new(),
        cache: HashMap::new()
    };
    lru.set(Entry {
        k: "key1".to_string(),
        v: "val1".to_string()
    });
    lru.set(Entry {
        k: "key2".to_string(),
        v: "val2".to_string()
    });
    dbg!(&lru);

    // Bug
    dbg!(lru.get("key1"));
    dbg!(&lru);
}
