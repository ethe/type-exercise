#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]

use crate::bitmap::{Bitmap, BitmapRef, BitmapRefMut};
use crate::dictionary::ListDictionary;
use crate::primitive::Primitive;

mod bitmap;
mod primitive;
mod dictionary;

pub trait Array {
    type ElementRef<'a> where Self: 'a;
    type ElementRefMut<'a> where Self: 'a;

    fn get(&self, id: usize) -> Option<Self::ElementRef<'_>>;
    fn append(&mut self, value: Self::ElementRef<'_>) -> usize;
}

#[derive(Debug)]
pub struct NullableFixedSizeListRefMut<'a, P: Primitive> {
    validity: BitmapRefMut<'a>,
    data: &'a mut [P],
}

#[derive(Debug)]
pub struct NullableFixedSizeListRef<'a, P: Primitive> {
    validity: BitmapRef<'a>,
    data: &'a [P],
}

#[derive(Debug)]
pub struct NullableFixedSizeListArray<P: Primitive> {
    validity: Bitmap,
    data: Vec<P>,
    list_size: usize,
}

#[derive(Debug)]
pub struct ListArray<P: Primitive> {
    offsets: Vec<usize>,
    data: Vec<P>,
}

impl<P: Primitive> Default for ListArray<P> {
    fn default() -> Self {
        Self {
            offsets: vec![0],
            data: Vec::<P>::new(),
        }
    }
}

impl<P: 'static + Primitive> Array for ListArray<P> {
    type ElementRef<'a> where Self: 'a = &'a [P] ;
    type ElementRefMut<'a> where Self: 'a = &'a mut [P];

    fn get(&self, id: usize) -> Option<Self::ElementRef<'_>> {
        let offset = self.offsets.get(id)?;
        let end = self.offsets.get(id + 1)?;
        Some(&self.data[*offset..*end])
    }

    fn append(&mut self, value: Self::ElementRef<'_>) -> usize {
        let id = self.offsets.len() - 1;
        let end = self.offsets[id] + value.len();
        self.offsets.push(end);
        self.data.extend_from_slice(value);
        id
    }
}

#[derive(Debug, Default)]
pub struct IdListArray<P: Primitive> {
    values: ListDictionary<P>,
    data: Vec<usize>,
}

impl<P: Primitive> IdListArray<P> {
    pub fn new() -> Self {
        Default::default()
    }
}
