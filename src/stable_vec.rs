use std::{
    alloc::Layout,
    cell::Cell,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use super::temp_vec::{LayoutCompatible, TransmutableContainer};

pub const DEFAULT_CHUNK_SIZE: usize = 64;

type Chunk<T, const CHUNK_SIZE: usize> = [MaybeUninit<T>; CHUNK_SIZE];

/// Like `Vec<T>`, but with chunked allocation and interior mutability.
/// Main differences:
/// + `push` only requires a `&self`, not a `&mut self` (!)
/// - always `!Sync` because of the interior mutabiltiy
/// + never has to copy elements, only the chunk pointer buffer has to grow
/// - no Deref to `&[T]`, because it's not contiguous
/// - many of `Vec<T>`'s convenience methods are missing
pub struct StableVec<T, const CHUNK_SIZE: usize = DEFAULT_CHUNK_SIZE> {
    data: Cell<NonNull<*mut Chunk<T, CHUNK_SIZE>>>,
    chunk_capacity: Cell<usize>,
    len: Cell<usize>,
}
// `!Sync` is guaranteed by the `data` pointer
unsafe impl<T: Send, const CHUNK_SIZE: usize> Send
    for StableVec<T, CHUNK_SIZE>
{
}

impl<T, const CHUNK_SIZE: usize> Default for StableVec<T, CHUNK_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for StableVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let res = Self::new();
        res.extend(iter);
        res
    }
}

impl<T, const CHUNK_SIZE: usize> StableVec<T, CHUNK_SIZE> {
    pub const fn new() -> Self {
        assert!(CHUNK_SIZE != 0);
        Self {
            data: Cell::new(NonNull::dangling()),
            chunk_capacity: Cell::new(if Self::holds_zsts() {
                usize::MAX / CHUNK_SIZE
            } else {
                0
            }),
            len: Cell::new(0),
        }
    }
    const fn holds_zsts() -> bool {
        std::mem::size_of::<T>() == 0
    }
    fn chunk_layout() -> Layout {
        Layout::new::<Chunk<T, CHUNK_SIZE>>()
    }
    fn chunk_pointer_array_layout(chunk_capacity: usize) -> Layout {
        Layout::array::<MaybeUninit<*mut Chunk<T, CHUNK_SIZE>>>(chunk_capacity)
            .unwrap()
    }
    pub fn with_capacity(cap: usize) -> Self {
        let v = Self::default();
        v.reserve(cap);
        v
    }
    pub fn capacity(&self) -> usize {
        self.chunk_capacity.get() * CHUNK_SIZE
    }
    pub fn len(&self) -> usize {
        self.len.get()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn clear(&mut self) {
        let prev_len = self.len.get();
        self.len.set(0);
        if !std::mem::needs_drop::<T>() {
            return;
        }
        let full_chunks = prev_len / CHUNK_SIZE;
        let remainder = prev_len % CHUNK_SIZE;
        unsafe {
            for i in 0..full_chunks {
                std::ptr::drop_in_place(
                    self.get_chunk_ptr_unchecked(i).cast::<[T; CHUNK_SIZE]>(),
                );
            }
            if remainder > 0 {
                std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
                    self.get_chunk_ptr_unchecked(full_chunks),
                    remainder,
                ));
            }
        }
    }
    pub fn reserve(&self, cap: usize) {
        let total_cap = cap + self.len.get();
        if total_cap <= self.capacity() {
            return;
        }
        let chunk_count_new = if total_cap == self.capacity() {
            self.chunk_capacity.get() * 2
        } else {
            total_cap.next_power_of_two().div_ceil(CHUNK_SIZE)
        };
        if Self::holds_zsts() {
            self.chunk_capacity.set(chunk_count_new);
            return;
        }
        unsafe {
            #[allow(clippy::cast_ptr_alignment)]
            let data = if self.chunk_capacity.get() == 0 {
                std::alloc::alloc(Self::chunk_pointer_array_layout(
                    chunk_count_new,
                ))
            } else {
                std::alloc::realloc(
                    self.data.get().as_ptr().cast(),
                    Self::chunk_pointer_array_layout(
                        self.chunk_capacity.get(),
                    ),
                    chunk_count_new,
                )
            }
            .cast::<*mut Chunk<T, CHUNK_SIZE>>();

            for i in self.chunk_capacity.get()..chunk_count_new {
                data.add(i)
                    .write(std::alloc::alloc(Self::chunk_layout()).cast());
            }
            self.data.set(NonNull::new_unchecked(data));
        }
        self.chunk_capacity.set(chunk_count_new);
    }
    pub fn push(&self, v: T) {
        let chunk_index = self.len.get() / CHUNK_SIZE;
        let offset = self.len.get() % CHUNK_SIZE;
        if chunk_index == self.chunk_capacity.get() {
            self.reserve(1);
        }
        unsafe {
            self.get_chunk_ptr_unchecked(chunk_index)
                .add(offset)
                .write(v);
        }
        self.len.set(self.len.get() + 1);
    }
    pub unsafe fn get_chunk_ptr_unchecked(&self, chunk_idx: usize) -> *mut T {
        unsafe { (*self.data.get().as_ptr().add(chunk_idx)).cast() }
    }
    pub unsafe fn get_element_pointer_unchecked(
        &self,
        index: usize,
    ) -> *mut T {
        unsafe {
            self.get_chunk_ptr_unchecked(index / CHUNK_SIZE)
                .add(index % CHUNK_SIZE)
        }
    }
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe { &*self.get_element_pointer_unchecked(index) }
    }
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { &mut *self.get_element_pointer_unchecked(index) }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if self.len() <= index {
            return None;
        }
        unsafe { Some(self.get_unchecked(index)) }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if self.len() <= index {
            return None;
        }
        unsafe { Some(self.get_unchecked_mut(index)) }
    }
    pub fn last(&self) -> Option<&T> {
        self.get(self.len().saturating_sub(1))
    }
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.len().saturating_sub(1))
    }
    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }
    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }
    pub fn iter(&self) -> StableVecIter<T, CHUNK_SIZE> {
        StableVecIter::new(self)
    }
    pub fn iter_mut(&mut self) -> StableVecIterMut<T, CHUNK_SIZE> {
        StableVecIterMut::new(self)
    }

    pub fn extend(&self, iter: impl IntoIterator<Item = T>) {
        // PERF: optimize this
        for elem in iter {
            self.push(elem);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut idx = self.len.get();
        if idx == 0 {
            return None;
        }
        idx -= 1;
        let chunk_index = idx / CHUNK_SIZE;
        let offset = idx % CHUNK_SIZE;
        let res = unsafe {
            self.get_chunk_ptr_unchecked(chunk_index).add(offset).read()
        };
        self.len.set(idx);
        Some(res)
    }
    // TODO: implement drain
}

impl<T, const CHUNK_SIZE: usize> Drop for StableVec<T, CHUNK_SIZE> {
    fn drop(&mut self) {
        if Self::holds_zsts() || self.chunk_capacity.get() == 0 {
            return;
        }
        self.clear();
        for i in 0..self.chunk_capacity.get() {
            unsafe {
                std::alloc::dealloc(
                    (*self.data.get().as_ptr().add(i)).cast(),
                    Self::chunk_layout(),
                );
            }
        }
        unsafe {
            std::alloc::dealloc(
                self.data.get().as_ptr().cast(),
                Self::chunk_pointer_array_layout(self.chunk_capacity.get()),
            );
        }
    }
}

impl<T: Clone, const CHUNK_SIZE: usize> Clone for StableVec<T, CHUNK_SIZE> {
    fn clone(&self) -> Self {
        if Self::holds_zsts() {
            return Self {
                data: self.data.clone(),
                chunk_capacity: self.chunk_capacity.clone(),
                len: self.len.clone(),
            };
        }
        if self.is_empty() {
            return Self::new();
        }
        // PERF: we could optimize this
        let res = Self::with_capacity(self.capacity());
        res.extend(self.iter().cloned());
        res
    }
}

impl<T> Index<usize> for StableVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<T> IndexMut<usize> for StableVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

pub struct StableVecIter<'a, T, const CHUNK_SIZE: usize> {
    vec: &'a StableVec<T, CHUNK_SIZE>,
    pos: usize,
}

impl<T, const CHUNK_SIZE: usize> Clone for StableVecIter<'_, T, CHUNK_SIZE> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec,
            pos: self.pos,
        }
    }
}

impl<'a, T, const CHUNK_SIZE: usize> StableVecIter<'a, T, CHUNK_SIZE> {
    pub fn new(vec: &'a StableVec<T, CHUNK_SIZE>) -> Self {
        Self { vec, pos: 0 }
    }
}

impl<'a, T, const CHUNK_SIZE: usize> Iterator
    for StableVecIter<'a, T, CHUNK_SIZE>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.pos;
        if idx == self.vec.len() {
            return None;
        }
        self.pos += 1;
        unsafe { Some(self.vec.get_unchecked(idx)) }
    }
}

impl<'a, T, const CHUNK_SIZE: usize> IntoIterator
    for &'a StableVec<T, CHUNK_SIZE>
{
    type Item = &'a T;
    type IntoIter = StableVecIter<'a, T, CHUNK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        StableVecIter::new(self)
    }
}

pub struct StableVecIterMut<'a, T, const CHUNK_SIZE: usize> {
    vec: &'a mut StableVec<T, CHUNK_SIZE>,
    pos: usize,
}

impl<'a, T, const CHUNK_SIZE: usize> StableVecIterMut<'a, T, CHUNK_SIZE> {
    pub fn new(vec: &'a mut StableVec<T, CHUNK_SIZE>) -> Self {
        Self { vec, pos: 0 }
    }
}

impl<'a, T, const CHUNK_SIZE: usize> Iterator
    for StableVecIterMut<'a, T, CHUNK_SIZE>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.pos;
        if idx == self.vec.len() {
            return None;
        }
        self.pos += 1;
        unsafe { Some(&mut *self.vec.get_element_pointer_unchecked(idx)) }
    }
}

impl<'a, T, const CHUNK_SIZE: usize> IntoIterator
    for &'a mut StableVec<T, CHUNK_SIZE>
{
    type Item = &'a mut T;
    type IntoIter = StableVecIterMut<'a, T, CHUNK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        StableVecIterMut::new(self)
    }
}

pub struct StableVecIntoIter<T, const CHUNK_SIZE: usize> {
    vec: ManuallyDrop<StableVec<T, CHUNK_SIZE>>,
    pos: usize,
}

impl<T, const CHUNK_SIZE: usize> StableVecIntoIter<T, CHUNK_SIZE> {
    pub fn new(vec: StableVec<T, CHUNK_SIZE>) -> Self {
        Self {
            vec: ManuallyDrop::new(vec),
            pos: 0,
        }
    }
    unsafe fn drop_remaining_elements(&self) {
        if !std::mem::needs_drop::<T>() {
            return;
        }
        let pos = self.pos;
        let len = self.vec.len();
        if pos == len {
            return;
        }
        let curr_chunk_rem = CHUNK_SIZE.min(len - pos) - pos % CHUNK_SIZE;
        let curr_chunk_end = pos + curr_chunk_rem;
        let first_full_chunk = curr_chunk_end / CHUNK_SIZE;
        let full_chunks = (len - curr_chunk_end) / CHUNK_SIZE;
        let full_chunks_end = curr_chunk_end + full_chunks * CHUNK_SIZE;
        let last_chunk_rem = len - full_chunks_end;
        unsafe {
            std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
                self.vec.get_element_pointer_unchecked(self.pos),
                curr_chunk_rem,
            ));
            for i in first_full_chunk..first_full_chunk + full_chunks {
                std::ptr::drop_in_place(
                    self.vec
                        .get_chunk_ptr_unchecked(i)
                        .cast::<[T; CHUNK_SIZE]>(),
                );
            }
            if last_chunk_rem > 0 {
                std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
                    self.vec.get_element_pointer_unchecked(full_chunks_end),
                    last_chunk_rem,
                ));
            }
        }
    }
    pub fn into_empty_vec(mut self) -> StableVec<T, CHUNK_SIZE> {
        unsafe {
            self.drop_remaining_elements();
            ManuallyDrop::take(&mut self.vec)
        }
    }
}

impl<T, const CHUNK_SIZE: usize> IntoIterator for StableVec<T, CHUNK_SIZE> {
    type Item = T;
    type IntoIter = StableVecIntoIter<T, CHUNK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        StableVecIntoIter::new(self)
    }
}

impl<T, const CHUNK_SIZE: usize> Iterator
    for StableVecIntoIter<T, CHUNK_SIZE>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.pos;
        if idx == self.vec.len() {
            return None;
        }
        self.pos += 1;
        unsafe {
            Some(std::ptr::read(self.vec.get_element_pointer_unchecked(idx)))
        }
    }
}

impl<T, const CHUNK_SIZE: usize> Drop for StableVecIntoIter<T, CHUNK_SIZE> {
    fn drop(&mut self) {
        unsafe {
            self.drop_remaining_elements();
            self.vec.len.set(0);
            let _ = ManuallyDrop::take(&mut self.vec);
        }
    }
}

pub fn transmute_stable_vec<T, U, const CHUNK_SIZE: usize>(
    src: StableVec<T, CHUNK_SIZE>,
) -> StableVec<U, CHUNK_SIZE> {
    #[allow(clippy::let_unit_value)]
    let () = LayoutCompatible::<T, U>::ASSERT_COMPATIBLE;

    let mut src = ManuallyDrop::new(src);

    src.clear();

    StableVec {
        data: unsafe {
            std::mem::transmute::<
                Cell<NonNull<*mut Chunk<T, CHUNK_SIZE>>>,
                Cell<NonNull<*mut Chunk<U, CHUNK_SIZE>>>,
            >(src.data.clone())
        },
        chunk_capacity: src.chunk_capacity.clone(),
        len: src.len.clone(),
    }
}

impl<T> TransmutableContainer for StableVec<T> {
    type ElementType = T;

    type ContainerType<Q> = StableVec<Q>;

    fn transmute<Q>(
        self,
    ) -> <Self as TransmutableContainer>::ContainerType<Q> {
        transmute_stable_vec(self)
    }

    fn transmute_from<Q>(
        src: <Self as TransmutableContainer>::ContainerType<Q>,
    ) -> Self {
        transmute_stable_vec(src)
    }
}
