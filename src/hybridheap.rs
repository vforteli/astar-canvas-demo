use std::{collections::HashMap, hash::Hash};

struct HeapItem<'a, K: Eq + Hash + PartialEq, V: Ord> {
    key: &'a K,
    value: V,
}

pub struct HybridHeap<'a, K: Eq + Hash + PartialEq, V: Ord> {
    items: Vec<HeapItem<'a, K, V>>,
    hashmap: HashMap<&'a K, usize>,
}

impl<'a, K: Eq + Hash + PartialEq, V: Ord> HybridHeap<'_, K, V> {
    pub fn new() -> Self {
        HybridHeap {
            items: Vec::with_capacity(1000), // todo fix...
            hashmap: HashMap::new(),
        }
    }

    /// Change value of a key, this will be bubbled up or down
    pub fn change_value(&self, key: &K, new_value: V) {
        todo!()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.hashmap.contains_key(&key)
    }

    pub fn get_value(&self, key: &K) -> V {
        todo!()
    }

    pub fn push(&mut self, key: &K, value: V) {
        /*
        if (hashmap.containsKey(object))
        {
            return;
            // Got the index to the old entry with the same key
            //old = heap[hashmap.get(node.object.toString())];
        }

        heap[tail] = new Item(value, object);
        int index = bubbleUp(tail);
        hashmap.put(object, index);
        tail++; */

        self.items.push(HeapItem { key, value });
        let new_index = self.bubble_up(self.items.len() - 1);

        self.hashmap.insert(key, new_index);
    }

    pub fn pop(&self) -> K {
        todo!()
    }

    pub fn peek(&self) -> K {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    fn bubble_up(&self, index: usize) -> usize {
        todo!()
    }

    fn bubble_down(&self, index: usize) -> usize {
        todo!()
    }
}
