
use std::ops::{AddAssign, SubAssign};
use std::iter::repeat;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Offset<T> {
    Inc(T),
    Dec(T)
}

pub(crate) struct Buffer<Entry> {
    data: Vec<Entry>,
    ptr: usize,
}

impl<Entry> Buffer<Entry>
    where Entry: Default + AddAssign + SubAssign + Clone + PartialOrd
{
    pub(crate) fn new(size: usize) -> Self {
        Self {
            ptr: 0,
            data: repeat(Entry::default())
                .take(size)
                .collect(),
        }
    }

    pub(crate) fn read(&self) -> &Entry {
        &self.data[self.ptr]
    }

    pub(crate) fn write(&mut self, val: Entry) {
        self.data[self.ptr] = val;
    }

    pub(crate) fn offset_ptr(&mut self, offset: Offset<usize>) {
        match offset {
            Offset::Inc(val) => self.ptr += val,
            Offset::Dec(val) => self.ptr -= val,
        }
    }

    pub(crate) fn offset_val(&mut self, offset: Offset<Entry>) {
        match offset {
            Offset::Inc(val) => self.data[self.ptr] += val,
            Offset::Dec(val) => self.data[self.ptr] -= val,
        }
    }
}
