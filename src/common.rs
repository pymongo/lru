pub mod prelude {
    pub use std::collections::HashMap;
    pub use std::hash::Hash;
    // pub use std::fmt::Debug;
    // warning: bounds on generic parameters are not enforced in type aliases

    pub trait Lru<K: Default + Hash + Eq + Clone, V: Default> {
        fn new(capacity: usize) -> Self;
        unsafe fn put(&mut self, key: K, val: V);
        unsafe fn get(&mut self, key: K) -> Option<V>;
    }    
}
