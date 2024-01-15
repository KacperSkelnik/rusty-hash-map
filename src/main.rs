use std::collections::hash_map::RandomState;
use std::collections::LinkedList;
use std::hash::{BuildHasher, Hash};

struct Entry<K, V> {
    key: K,
    value: V,
}

struct HashMap<K, V> {
    random_state: RandomState,
    size: u64,
    entries: Vec<LinkedList<Entry<K, V>>>,
    elements: u64,
}

impl<K, V> HashMap<K, V>
where
    K: Eq,
    K: Hash,
    V: Clone,
{
    fn get_id(&self, key: &K) -> usize {
        let hash = self.random_state.hash_one(key);
        (hash % self.size) as usize
    }

    fn resize(&mut self) {
        self.size = self.size * 2;
        self.entries
            .resize_with(self.size as usize, || LinkedList::new());
    }

    pub fn new() -> Self {
        HashMap::<K, V> {
            random_state: RandomState::new(),
            size: 2,
            entries: Vec::from([LinkedList::new(), LinkedList::new()]),
            elements: 0,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.elements >= self.size as u64 {
            self.resize()
        }

        let id = self.get_id(&key);

        let mut already_in_entries = false;
        self.entries[id].iter_mut().for_each(|entry| {
            if entry.key == key {
                entry.value = value.clone();
                already_in_entries = true
            }
        });

        if !already_in_entries {
            self.entries[id].push_back(Entry {
                key,
                value: value.clone(),
            });
            self.elements += 1;
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let id = self.get_id(&key);
        self.entries[id]
            .iter()
            .find(|entry| entry.key == key)
            .map(|e| &e.value)
    }

    pub fn remove(&mut self, key: K) {
        let id = self.get_id(&key);
        let inner_id = self.entries[id].iter().position(|entry| entry.key == key);
        match inner_id {
            None => (),
            Some(inner) => {
                let mut rest = self.entries[id].split_off(inner);
                rest.pop_front();
                self.entries[id].append(&mut rest);
            }
        }
    }
}

fn main() {
    let mut hash_map = HashMap::<i32, i32>::new();
    hash_map.put(1, 1);
    hash_map.put(2, 2);
    println!("{:?}", hash_map.get(1));
    println!("{:?}", hash_map.get(3));
    hash_map.put(2, 1);
    println!("{:?}", hash_map.get(2));
    hash_map.remove(2);
    println!("{:?}", hash_map.get(2));
}
