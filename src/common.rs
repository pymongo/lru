pub mod prelude {
    pub use std::collections::HashMap;
    // warning: bounds on generic parameters are not enforced in type aliases

    pub trait Lru<K, V> {
        fn new(capacity: usize) -> Self;
        fn put(&mut self, key: K, val: V);
        fn get(&mut self, key: K) -> Option<V>;
    }    
}
