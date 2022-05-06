use std::hash::{BuildHasher, Hash, Hasher};
use ahash::RandomState;
use hashbrown::hash_map::RawEntryMut;
use hashbrown::HashMap;
use crate::{Array, ListArray, Primitive};

fn hash_with_state<H: Hash>(state: &RandomState, value: H) -> u64 {
    let mut hasher = state.build_hasher();
    value.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Default)]
pub struct ListDictionary<P: Primitive> {
    hash_state: RandomState,
    dedup: HashMap<usize, (), ()>,
    data: ListArray<P>,
}

impl<P: Primitive + Hash> ListDictionary<P>  {
    fn new() -> Self {
        Default::default()
    }

    pub(crate) fn lookup_or_insert(&mut self, value: &[P]) -> usize {
        let hash = hash_with_state(&self.hash_state, value);
        let entry = self
            .dedup
            .raw_entry_mut()
            .from_hash(hash, |key| {
                value == self.data.get(*key).unwrap()
            });

        return match entry {
            RawEntryMut::Occupied(entry) => *entry.into_key(),
            RawEntryMut::Vacant(entry) => {
                let index = self.data.append(value);
                *entry
                    .insert_with_hasher(hash, index, (), |index| {
                        let list = self.data.get(*index).unwrap();
                        hash_with_state(&self.hash_state, list)
                    })
                    .0
            }
        } + 1;
    }

    pub(crate) fn lookup(&self, value: &[P]) -> Option<usize> {
        return self
            .dedup
            .raw_entry()
            .from_hash(hash_with_state(&self.hash_state, value), |key| {
                value == self.data.get(*key).unwrap()
            })
            .map(|(&symbol, &())| symbol + 1);
    }

    pub(crate) fn get(&self, id: usize) -> Option<&[P]> {
        if id == 0 {
            None
        } else {
            self.data.get(id - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ListDictionary;

    #[test]
    fn test_storage() {
        let mut dict = ListDictionary::<u8>::new();
        let id = dict.lookup_or_insert("hello, world".as_ref());
        let id2 = dict.lookup_or_insert("hello, world".as_ref());
        assert_eq!(id, id2);
        let id3 = dict.lookup_or_insert("hello world".as_ref());
        assert_ne!(id, id3);
        let v1 = dict.get(id);
        let v2 = dict.get(id2);
        assert_eq!(v1, v2);
    }
}
