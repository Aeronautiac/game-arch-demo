/*
* This data structure is intended to solve the problem of entities holding variable numbers of items
* within components leading to frequent cache misses. Rather than holding their own individual heap
* allocation, they hold the head of their list in some list arena.
*/

use slotmap::DenseSlotMap;

struct ListArenaNode<K: slotmap::Key, T> {
    data: T,
    next_sibling: Option<K>,
    prev_sibling: Option<K>,
}

// there may be cases where entities have many items in their list, leaving massive gaps after they are
// removed, and slowly degrading the integrity of a standard slotmap, so dense is the better option.
// if for example there are many entities with modifiers, and those entities are all removed, you
// would get tons of cache misses for the entities that remain.
pub struct ListArena<K: slotmap::Key, T> {
    nodes: DenseSlotMap<K, ListArenaNode<K, T>>,
}

impl<K: slotmap::Key, T> ListArena<K, T> {
    pub fn new() -> Self {
        ListArena {
            nodes: DenseSlotMap::default(),
        }
    }

    pub fn get(&mut self, key: K) -> Option<&T> {
        if let Some(node) = self.nodes.get(key) {
            Some(&node.data)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut T> {
        if let Some(node) = self.nodes.get_mut(key) {
            Some(&mut node.data)
        } else {
            None
        }
    }

    pub fn add(&mut self, head: &mut Option<K>, value: T) -> K {
        let node = ListArenaNode {
            data: value,
            next_sibling: *head,
            prev_sibling: None,
        };
        let k = self.nodes.insert(node);
        if let Some(head_id) = head
            && let Some(head_node) = self.nodes.get_mut(*head_id)
        {
            head_node.prev_sibling = Some(k);
        }
        *head = Some(k);
        k
    }

    // returns the data if the key mapped to a value
    pub fn remove(&mut self, head: &mut Option<K>, key: K) -> Option<T> {
        if let Some(node) = self.nodes.remove(key) {
            // removal may otherwise invalidate the head
            if let Some(head_key) = head
                && *head_key == key
            {
                *head = node.next_sibling;
            }

            if let Some(prev) = node.prev_sibling {
                let prev_node = self
                    .nodes
                    .get_mut(prev)
                    .expect("sibling pointer to invalid node");
                prev_node.next_sibling = node.next_sibling;
            }

            if let Some(next) = node.next_sibling {
                let next_node = self
                    .nodes
                    .get_mut(next)
                    .expect("sibling pointer to invalid node");
                next_node.prev_sibling = node.prev_sibling;
            }

            Some(node.data)
        } else {
            None
        }
    }

    pub fn for_each_mut<F>(&mut self, start: K, mut f: F)
    where
        F: FnMut(&mut T),
    {
        let mut curr = Some(start);
        while let Some(key) = curr
            && let Some(node) = self.nodes.get_mut(key)
        {
            f(&mut node.data);
            curr = node.next_sibling;
        }
    }

    pub fn for_each<F>(&self, start: K, f: F)
    where
        F: Fn(&T),
    {
        let mut curr = Some(start);
        while let Some(key) = curr
            && let Some(node) = self.nodes.get(key)
        {
            f(&node.data);
            curr = node.next_sibling;
        }
    }
}

impl<K: slotmap::Key, T> Default for ListArena<K, T> {
    fn default() -> Self {
        ListArena::new()
    }
}

#[cfg(test)]
mod list_arena_tests {
    use super::*;
    use slotmap::DefaultKey;

    fn collect_values(
        arena: &mut ListArena<DefaultKey, i32>,
        head: Option<DefaultKey>,
    ) -> Vec<i32> {
        let mut out = Vec::new();
        if let Some(head) = head {
            arena.for_each_mut(head, |v| out.push(*v));
        }
        out
    }

    #[test]
    fn add_sets_head_and_next_sibling() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;

        let a = arena.add(&mut head, 10);
        let b = arena.add(&mut head, 20);
        let c = arena.add(&mut head, 30);

        assert_eq!(head, Some(c));
        assert_eq!(collect_values(&mut arena, head), vec![30, 20, 10]);
        assert_eq!(arena.get(a), Some(&10));
        assert_eq!(arena.get(b), Some(&20));
        assert_eq!(arena.get(c), Some(&30));
    }

    #[test]
    fn get_mut_allows_modification() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        let key = arena.add(&mut head, 5);

        *arena.get_mut(key).unwrap() = 99;
        assert_eq!(arena.get(key), Some(&99));
    }

    #[test]
    fn get_and_get_mut_return_none_for_unknown_key() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        let key = arena.add(&mut head, 1);

        let removed = arena.remove(&mut head, key);
        assert_eq!(removed, Some(1));
        assert_eq!(arena.get(key), None);
        assert_eq!(arena.get_mut(key), None);
    }

    #[test]
    fn remove_updates_head_when_removing_head() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        let first = arena.add(&mut head, 10);
        arena.add(&mut head, 20);

        let old_head = head.unwrap();
        let removed = arena.remove(&mut head, old_head);
        assert_eq!(removed, Some(20));
        assert_eq!(head, Some(first));
        assert_eq!(collect_values(&mut arena, head), vec![10]);
    }

    #[test]
    fn remove_middle_preserves_remaining_order() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        let a = arena.add(&mut head, 10);
        let b = arena.add(&mut head, 20);
        let c = arena.add(&mut head, 30);
        // list order: c -> b -> a

        let removed = arena.remove(&mut head, b);
        assert_eq!(removed, Some(20));
        assert_eq!(collect_values(&mut arena, head), vec![30, 10]);
        assert_eq!(arena.get(a), Some(&10));
        assert_eq!(arena.get(c), Some(&30));
        assert_eq!(arena.get(b), None);
    }

    #[test]
    fn remove_tail_preserves_remaining_order() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        let a = arena.add(&mut head, 10);
        let b = arena.add(&mut head, 20);
        let c = arena.add(&mut head, 30);
        // list order: c -> b -> a

        let removed = arena.remove(&mut head, a);
        assert_eq!(removed, Some(10));
        assert_eq!(collect_values(&mut arena, head), vec![30, 20]);
        assert_eq!(arena.get(b), Some(&20));
        assert_eq!(arena.get(c), Some(&30));
    }

    #[test]
    fn remove_unknown_key_returns_none_and_leaves_list() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        let a = arena.add(&mut head, 10);
        let b = arena.add(&mut head, 20);
        arena.remove(&mut head, a);

        let removed = arena.remove(&mut head, a);
        assert_eq!(removed, None);
        assert_eq!(collect_values(&mut arena, head), vec![20]);
        assert_eq!(arena.get(b), Some(&20));
    }

    #[test]
    fn remove_last_item_clears_head() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        let key = arena.add(&mut head, 42);

        let removed = arena.remove(&mut head, key);
        assert_eq!(removed, Some(42));
        assert_eq!(head, None);
    }

    #[test]
    fn for_each_mut_modifies_all_items() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        arena.add(&mut head, 1);
        arena.add(&mut head, 2);
        arena.add(&mut head, 3);

        arena.for_each_mut(head.unwrap(), |v| *v *= 10);
        assert_eq!(collect_values(&mut arena, head), vec![30, 20, 10]);
    }

    #[test]
    fn for_each_can_track_processed_count() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        arena.add(&mut head, 1);
        arena.add(&mut head, 2);
        arena.add(&mut head, 3);

        let mut count = 0;
        arena.for_each_mut(head.unwrap(), |_| count += 1);
        assert_eq!(count, 3);
    }

    #[test]
    fn for_each_on_removed_start_does_nothing() {
        let mut arena: ListArena<DefaultKey, i32> = ListArena::new();
        let mut head = None;
        let key = arena.add(&mut head, 1);
        arena.remove(&mut head, key);

        arena.for_each(key, |_| panic!("should not be called"));
    }
}
