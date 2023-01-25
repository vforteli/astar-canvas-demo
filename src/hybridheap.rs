use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Copy, Debug)]
struct HeapItem<K: Eq + Hash + PartialEq, V: PartialOrd> {
    key: K,
    value: V,
}

pub struct HybridHeap<'a, K: Eq + Hash + PartialEq, V: PartialOrd + Copy> {
    items: Vec<HeapItem<K, V>>,
    hashmap: HashMap<&'a K, usize>,
}

impl<'a, K: Eq + Hash + PartialEq, V: PartialOrd + Copy> HybridHeap<'a, K, V> {
    pub fn new() -> Self {
        HybridHeap {
            items: Vec::with_capacity(1000),
            hashmap: HashMap::new(),
        }
    }

    /// Change value of a key already in heap, this will be bubbled up or down
    pub fn change_value(&mut self, key: &K, new_value: V) {
        let index = self.hashmap.get(key).unwrap();

        let item = self.items.get(*index).unwrap();

        if item.value > new_value {
            self.items[*index] = HeapItem {
                key: item.key,
                value: new_value,
            };
            Some(self.bubble_up(*index))
        } else if item.value < new_value {
            self.items[*index] = HeapItem {
                key: item.key,
                value: new_value,
            };
            Some(self.bubble_down(*index))
        } else {
            None
        };
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.hashmap.contains_key(&key)
    }

    pub fn get_value(self, key: &K) -> V {
        todo!()
    }

    /// Push new item with value
    pub fn push(&mut self, key: K, value: V) {
        self.items.push(HeapItem { key, value });
        self.bubble_up(self.items.len() - 1);
    }

    /// Pop item
    pub fn pop(&mut self) -> Option<K> {
        match self.items.get(0) {
            Some(item) => {
                let key = item.key;
                self.hashmap.remove(&item.key);

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

    /// Peek item without removing it
    pub fn peek(&self) -> Option<&K> {
        match self.items.get(0) {
            Some(item) => Some(&item.key),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn bubble_up(&mut self, index: usize) {
        let mut index = index;
        let value = self.items.get(index).unwrap().value;
        let last = self.items.get(index).unwrap().key;
        let mut parent_index = if index > 1 { (index - 1) / 2 } else { 0 };

        while index > 0 && self.items.get(parent_index).unwrap().value >= value {
            self.hashmap
                .insert(&self.items.get(parent_index).unwrap().key, index);

            self.items.swap(index, parent_index);
            index = parent_index;
            if index < 1 {
                break;
            }

            parent_index = (index - 1) / 2;
        }

        self.items[index] = HeapItem { key: last, value };
        self.hashmap.insert(&last, index);
    }

    fn bubble_down(&mut self, index: usize) {
        let mut index = index;
        let foo = self.items.get(index).unwrap().value;
        let bar = self.items.get(index).unwrap().key;

        while index < self.items.len() / 2 {
            let left_child_index = 2 * index + 1;
            let right_child_index = left_child_index + 1;

            let smaller_node_index = if right_child_index < self.items.len()
                && (self.items.get(right_child_index).unwrap().value
                    < self.items.get(left_child_index).unwrap().value)
            {
                right_child_index
            } else {
                left_child_index
            };

            if foo <= self.items.get(smaller_node_index).unwrap().value {
                break;
            }

            self.hashmap
                .insert(&self.items.get(smaller_node_index).unwrap().key, index);
            self.items.swap(index, smaller_node_index);
            index = smaller_node_index;
        }

        self.items[index] = HeapItem {
            key: bar,
            value: foo,
        };
        self.hashmap.insert(&bar, index);
    }
}

#[cfg(test)]
mod tests {

    #[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
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

        heap.push(some_item, 2);

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

        heap.push(some_item, 2);

        let actual = heap.pop();

        assert_eq!(Some(&some_item), actual);

        assert!(heap.pop().is_none());
    }

    #[test]
    fn pop_many() {
        let mut heap = HybridHeap::new();

        let some_item = TestItem { some_value: 2 };
        let some_other_item = TestItem { some_value: 1 };

        heap.push(some_item, 2);
        heap.push(some_other_item, 1);

        let actual = heap.pop();

        assert_eq!(Some(&some_other_item), actual);
    }

    #[test]
    fn test_delete_min() {
        let mut heap = HybridHeap::new();
        assert!(heap.is_empty());
        assert!(heap.peek().is_none());
        assert!(heap.pop().is_none());

        heap.push("first", 10);
        heap.push("second", 5);
        heap.push("third", 15);

        assert!(!heap.is_empty());

        let peek_actual = heap.peek();
        assert_eq!(Some(&"second"), peek_actual);

        let actual = heap.pop();
        assert_eq!(Some(&"second"), actual);

        let peek_actual2 = heap.peek();
        assert_eq!(Some(&"first"), peek_actual2);

        let actual2 = heap.pop();
        assert_eq!(Some(&"first"), actual2);

        let peek_actual3 = heap.peek();
        assert_eq!(Some(&"third"), peek_actual3);

        let actual3 = heap.pop();
        assert_eq!(Some(&"third"), actual3);

        assert!(heap.is_empty());
        assert!(heap.peek().is_none());
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_change_value_up() {
        let mut heap = HybridHeap::new();

        heap.push("first", 10);
        heap.push("second", 5);
        heap.push("third", 15);

        assert_eq!(Some(&"second"), heap.peek());

        heap.change_value(&"third", 2);
        assert_eq!(Some(&"third"), heap.peek());

        heap.change_value(&"first", 1);
        assert_eq!(Some(&"first"), heap.peek());
    }

    #[test]
    fn test_change_value_down() {
        let mut heap = HybridHeap::new();

        heap.push("first", 10);
        heap.push("second", 5);
        heap.push("third", 15);

        assert_eq!(Some(&"second"), heap.peek());
        println!("{:?}", heap.items);
        println!("{:?}", heap.hashmap);

        heap.change_value(&"second", 100);
        println!("{:?}", heap.items);
        println!("{:?}", heap.hashmap);
        assert_eq!(Some(&"first"), heap.peek());
    }

    #[test]
    fn test_insert_lower_value() {
        let mut heap = HybridHeap::new();

        heap.push("first", 2);
        heap.push("second", 2);

        assert_eq!(Some(&"second"), heap.peek());
        assert_eq!(&"second", heap.items.get(0).unwrap().key);
        assert_eq!(&"first", heap.items.get(1).unwrap().key);

        println!("{:?}", heap.items);
        println!("{:?}", heap.hashmap);

        assert_eq!(&0, heap.hashmap.get(&"second").unwrap());
        assert_eq!(&1, heap.hashmap.get(&"first").unwrap());
    }
}
