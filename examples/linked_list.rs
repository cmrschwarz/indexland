use indexland::{Idx, IndexSlice, IndexVec};

#[cfg(not(feature = "nonmax"))]
#[derive(Idx)]
pub struct NodeId(u32);

#[cfg(feature = "nonmax")]
#[derive(Idx)]
/// using [`NonMax<u32>`] keeps [Option<NodeId>] at 4 bytes
pub struct NodeId(indexland::NonMax<u32>);

#[derive(Default, Clone, Debug)]
pub struct Node<T> {
    data: T,
    prev: Option<NodeId>,
    next: Option<NodeId>,
}

#[derive(Default, Clone, Debug)]
pub struct LinkedList<T> {
    nodes: IndexVec<NodeId, Node<T>>,
    head: Option<NodeId>,
    tail: Option<NodeId>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            nodes: IndexVec::new(),
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, data: T) {
        let node = Node {
            data,
            prev: self.tail,
            next: None,
        };
        let new_id = self.nodes.push_get_id(node);

        if let Some(tail) = self.tail {
            self.nodes[tail].next = Some(new_id);
        } else {
            self.head = Some(new_id);
        }
        self.tail = Some(new_id);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let tail_id = self.tail?;

        let node = self.nodes.remove(tail_id);
        self.tail = node.prev;

        if let Some(new_tail) = self.tail {
            self.nodes[new_tail].next = None;
        } else {
            self.head = None;
        }

        Some(node.data)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
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

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    println!("List contents:");
    for item in list.iter() {
        println!("{}", item);
    }
}
