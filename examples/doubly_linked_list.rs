//! NOTE: this is a practical example. If you're looking for more of an
//! api listing check out construction.rs and iteration.rs instead.
//!
//!
//! This implements a basic doubly linked list using a growing array as the
//! underlying data structure.
//!
//! Nothing special, but it demonstrates the use of [`IndexVec`] to easily
//! achieve a pattern that's typically considered
//! challenging to do in stable rust due to the borrow checker.
//!
//! The implementation details are mostly irrelevant, this is just meant to
//! show a few indexland features, each highlighted with `// NOTE:`.

use indexland::{Idx, IndexSlice, IndexVec, NonMax};

// NOTE: Using [`NonMax<u32>`] ensures that [`Option<NodeId>`] is 4 bytes instead of
// 8, which is a big perf improvement for this usecase.
#[derive(Idx)]
pub struct NodeId(NonMax<u32>);

#[derive(Default, Clone, Debug)]
pub struct LinkedList<T> {
    // NOTE: immediately tells the reader much more than just `Vec<Node<T>>`.
    // While it may have been pretty obvious what the intention was
    // in this simple case, the explicit NodeId
    // eliminates any doubt and lets the reader move on confidently.
    nodes: IndexVec<NodeId, Node<T>>,
    head: Option<NodeId>,
    tail: Option<NodeId>,
}

#[derive(Default, Clone, Debug)]
pub struct Node<T> {
    data: T,
    prev: Option<NodeId>,
    next: Option<NodeId>,
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            // NOTE: all indexland containers have zero alloc const new!
            // (caveat on HashMap explained in the doc.)
            nodes: IndexVec::new(),
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, data: T) -> NodeId {
        let node = Node {
            data,
            prev: self.tail,
            next: None,
        };

        // NOTE: `push_get_idx` is a nice convenience helper
        let new_id = self.nodes.push_get_idx(node);

        if let Some(tail) = self.tail {
            self.nodes[tail].next = Some(new_id);
        } else {
            self.head = Some(new_id);
        }

        self.tail = Some(new_id);

        new_id
    }

    /// O(1) remove, one of the few reasons anybody would ever want to use a
    /// linked list in the first place.
    pub fn remove(&mut self, idx: NodeId) -> T {
        // NOTE: index based swap remove, otherwise same api we all know and love.
        let node = self.nodes.swap_remove(idx);

        // Update adjacent nodes
        match (node.prev, node.next) {
            (Some(prev), Some(next)) => {
                self.nodes[prev].next = Some(next);
                self.nodes[next].prev = Some(prev);
            }
            (Some(prev), None) => self.nodes[prev].next = None,
            (None, Some(next)) => self.nodes[next].prev = None,
            (None, None) => {}
        }

        // If the removed node wasn't the last one
        // update the moved node's adjacent nodes

        // NOTE: `len_idx` is a very common and useful convenience helper.
        // It returns the index that a node after the last (index `len()`)
        // would have.
        if idx < self.nodes.len_idx() {
            if let Some(prev) = self.nodes[idx].prev {
                self.nodes[prev].next = Some(idx);
            }
            if let Some(next) = self.nodes[idx].next {
                self.nodes[next].prev = Some(idx);
            }
        }

        // Update head/tail
        if self.head == Some(self.nodes.len_idx()) {
            self.head = Some(idx);
        } else if self.head == Some(idx) {
            self.head = node.next;
        }

        if self.tail == Some(self.nodes.len_idx()) {
            self.tail = Some(idx);
        } else if self.tail == Some(idx) {
            self.tail = node.next;
        }

        node.data
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            // NOTE: `as_index_slice` is identical to
            // `&*self.nodes` (using IndexVec's Deref to IndexSlice),
            // but a bit more clear. we offer `.as_slice()` aswell to get
            // a non index aware slice. We don't judge :).
            nodes: self.nodes.as_slice(),
            current: self.head,
        }
    }
}

pub struct Iter<'a, T> {
    nodes: &'a IndexSlice<NodeId, Node<T>>,
    current: Option<NodeId>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.current?;

        self.current = self.nodes[curr].next;

        Some(&self.nodes[curr].data)
    }
}

// Simple Demo:
fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    let second_id = list.push_back(42);
    list.push_back(2);
    list.push_back(3);

    list.remove(second_id);

    println!("List contents:");
    for item in list.iter() {
        println!("{item}");
    }
}
