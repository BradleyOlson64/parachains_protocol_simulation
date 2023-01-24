
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod happy_path;
mod unhappy_paths;

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}