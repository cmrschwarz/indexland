use indexland::{Idx, IndexSlice, IndexVec};

#[derive(Idx)]
pub struct NodeId(indexland::NonMax<u32>);

/// This is a very standard linked list implemenation using a growing
/// array as the underlying data structure.
/// Nothing special really, but it demonstrates the use of [`IndexVec`] and
/// [`IndexSlice`].
///
/// The implementation details aren't inportant, I don't want to waste
/// your time studying them.
///
/// This is just a random usecase example that's meant for you to see and
/// enjoy some of the neat indexland helpers (highlighted by // NOTE)s.
#[derive(Default, Clone, Debug)]
pub struct LinkedList<T> {
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

        // NOTE: `push_get_id` is a nice convenience helper
        let new_id = self.nodes.push_get_id(node);

        if let Some(tail) = self.tail {
            self.nodes[tail].next = Some(new_id);
        } else {
            self.head = Some(new_id);
        }

        self.tail = Some(new_id);

        new_id
    }

    /// O(1) remove, one of the few reasons anybody would ever want to use a
    /// linked list in the first place (as anything but an example).
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

        // NOTE: `len_idx` is a nice convenience helper.
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

    pub fn iter(&self) -> Iter<T> {
        Iter {
            // NOTE: `as_index_slice` is identical to
            // `&*self.nodes` (using IndexVec's Deref to IndexSlice),
            // but a bit more clear. we offer `.as_slice()` aswell to get
            // a non index aware slice. We don't judge :).
            nodes: self.nodes.as_index_slice(),
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

// phew! below is a simple demo usage of our LinkedList:

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    let second_id = list.push_back(2);
    list.push_back(3);
    list.push_back(4);

    list.remove(second_id);

    println!("List contents:");
    for item in list.iter() {
        println!("{}", item);
    }
}
