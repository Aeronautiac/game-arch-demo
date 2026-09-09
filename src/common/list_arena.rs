use slotmap::DenseSlotMap;

/*
* This data structure is intended to solve the problem of entities holding variable numbers of items
* within components leading to frequent cache misses. Rather than holding their own individual heap
* allocation, they hold the head of their list in some list arena.
*/

struct ListArenaNode<K: slotmap::Key, T> {
    data: T,
    next_sibling: Option<K>,
    prev_sibling: Option<K>,
}

// there may be cases where entities have many items in their list, leaving massive gaps after they are
// removed, and slowly degrading the integrity of a standard slotmap, so dense is the better option.
pub struct ListArena<K: slotmap::Key, T> {
    nodes: DenseSlotMap<K, ListArenaNode<K, T>>,
}

impl<K: slotmap::Key, T> ListArena<K, T> {}

// pub struct ListArenaIterator {}
//
// impl Iterator for ListArenaIterator {
//     type Item = ;
// }
