use std::{collections::HashMap, hash::Hash};

struct HeapItem<'a, K: Eq + Hash + PartialEq, V: Ord + PartialOrd> {
    key: &'a K,
    value: V,
}

pub struct HybridHeap<'a, K: Eq + Hash + PartialEq, V: Ord> {
    items: Vec<HeapItem<'a, K, V>>,
    hashmap: HashMap<&'a K, usize>,
}

impl<'a, K: Eq + Hash + PartialEq, V: Ord> HybridHeap<'a, K, V> {
    pub fn new() -> Self {
        HybridHeap {
            items: Vec::with_capacity(1000),
            hashmap: HashMap::new(),
        }
    }

    /// Change value of a key, this will be bubbled up or down
    pub fn change_value(self, key: &K, new_value: V) {
        todo!()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.hashmap.contains_key(&key)
    }

    pub fn get_value(self, key: &K) -> V {
        todo!()
    }

    pub fn push(&mut self, key: &'a K, value: V) {
        self.items.push(HeapItem { key, value });
        let new_index = self.bubble_up(self.items.len() - 1);
        self.hashmap.insert(key, new_index);
    }

    pub fn pop(&mut self) -> Option<&K> {
        match self.items.get(0) {
            Some(item) => {
                let key = item.key;
                self.hashmap.remove(item.key);

                if let Some(last) = self.items.pop() {
                    if self.items.len() > 0 {
                        self.items[0] = last;
                        self.bubble_down(0);
                    };
                };

                Some(key)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&K> {
        match self.items.get(0) {
            Some(item) => Some(item.key),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn bubble_up(&mut self, index: usize) -> usize {
        let item = self.items.get(index).unwrap();

        let parent_index = (index - 1) / 2;

        while index > 0 && &self.items.get(parent_index).unwrap().value >= &item.value {
            let parent = &self.items[parent_index];
            self.items[index] = parent;
            /* hashmap.put(heap[parentindex].value, index);
            heap[index] = heap[parentindex];
            index = parentindex;
            parentindex = (index - 1) / 2; */
        }
        /*

        heap[index] = last;
        hashmap.put(last.value, index);
        return index;
         */
        1
    }

    fn bubble_down(&mut self, index: usize) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {

    #[derive(Eq, PartialEq, Hash, Debug)]
    struct TestItem {
        some_value: u32,
    }

    use super::*;

    #[test]
    fn push_one() {
        let mut heap = HybridHeap::new();

        let some_item = TestItem { some_value: 1 };

        heap.push(&some_item, 1);

        assert_eq!(false, heap.is_empty())
    }

    #[test]
    fn push_many() {
        let mut heap = HybridHeap::new();

        let some_item = TestItem { some_value: 1 };
        let some_other_item = TestItem { some_value: 2 };

        heap.push(&some_item, 2);
        heap.push(&some_other_item, 1);

        assert_eq!(false, heap.is_empty())
    }

    #[test]
    fn push_peek() {
        let mut heap = HybridHeap::new();

        let some_item = TestItem { some_value: 2 };

        heap.push(&some_item, 2);

        let actual = heap.peek();

        match actual {
            Some(item) => {
                assert_eq!(2, item.some_value)
            }
            None => assert!(false),
        };

        assert_eq!(false, heap.is_empty());
    }

    #[test]
    fn contains_key() {
        let mut heap = HybridHeap::new();

        let some_item = TestItem { some_value: 2 };

        let some_other_item = TestItem { some_value: 1 };

        heap.push(&some_item, 2);

        assert_eq!(true, heap.contains_key(&some_item));
        assert_eq!(false, heap.contains_key(&some_other_item));
    }

    #[test]
    fn pop_empty() {
        let mut heap: HybridHeap<TestItem, u32> = HybridHeap::new();
        let actual = heap.pop();

        assert!(actual.is_none());
    }

    #[test]
    fn pop_one() {
        let mut heap = HybridHeap::new();

        let some_item = TestItem { some_value: 2 };

        heap.push(&some_item, 2);

        let actual = heap.pop();

        assert_eq!(Some(&some_item), actual);

        assert!(heap.pop().is_none());
    }

    #[test]
    fn pop_many() {
        let mut heap = HybridHeap::new();

        let some_item = TestItem { some_value: 2 };
        let some_other_item = TestItem { some_value: 1 };

        heap.push(&some_item, 2);
        heap.push(&some_other_item, 1);

        let actual = heap.pop();

        assert_eq!(Some(&some_other_item), actual);
    }
}
