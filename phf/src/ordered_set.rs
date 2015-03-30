//! An order-preserving immutable set constructed at compile time.
use std::prelude::v1::*;
use std::borrow::Borrow;
use std::iter::{IntoIterator, RandomAccessIterator};
use std::fmt;
use ordered_map;
use {PhfHash, OrderedMap};

/// An order-preserving immutable set constructed at compile time.
///
/// Unlike a `Set`, iteration order is guaranteed to match the definition
/// order.
///
/// ## Note
///
/// The fields of this struct are public so that they may be initialized by the
/// `phf_ordered_set!` macro and code generation. They are subject to change at
/// any time and should never be accessed directly.
pub struct OrderedSet<T:'static> {
    #[doc(hidden)]
    pub map: OrderedMap<T, ()>,
}

impl<T> fmt::Debug for OrderedSet<T> where T: fmt::Debug {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = fmt.debug_set();
        for entry in self {
            builder = builder.entry(entry);
        }
        builder.finish()
    }
}

impl<T> OrderedSet<T> {
    /// Returns the number of elements in the `OrderedSet`.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the `OrderedSet` contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the set's internal static instance of the given
    /// key.
    ///
    /// This can be useful for interning schemes.
    pub fn get_key<U: ?Sized>(&self, key: &U) -> Option<&T> where U: Eq + PhfHash, T: Borrow<U> {
        self.map.get_key(key)
    }

    /// Returns the index of the key within the list used to initialize
    /// the ordered set.
    pub fn get_index<U: ?Sized>(&self, key: &U) -> Option<usize>
            where U: Eq + PhfHash, T: Borrow<U> {
        self.map.get_index(key)
    }

    /// Returns true if `value` is in the `Set`.
    pub fn contains<U: ?Sized>(&self, value: &U) -> bool where U: Eq + PhfHash, T: Borrow<U> {
        self.map.contains_key(value)
    }

    /// Returns an iterator over the values in the set.
    ///
    /// Values are returned in the same order in which they were defined.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { iter: self.map.keys() }
    }
}

impl<T> OrderedSet<T> where T: Eq + PhfHash {
    /// Returns true if `other` shares no elements with `self`.
    #[inline]
    pub fn is_disjoint(&self, other: &OrderedSet<T>) -> bool {
        !self.iter().any(|value| other.contains(value))
    }

    /// Returns true if `other` contains all values in `self`.
    #[inline]
    pub fn is_subset(&self, other: &OrderedSet<T>) -> bool {
        self.iter().all(|value| other.contains(value))
    }

    /// Returns true if `self` contains all values in `other`.
    #[inline]
    pub fn is_superset(&self, other: &OrderedSet<T>) -> bool {
        other.is_subset(self)
    }
}

impl<'a, T> IntoIterator for &'a OrderedSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

/// An iterator over the values in a `OrderedSet`.
pub struct Iter<'a, T:'a> {
    iter: ordered_map::Keys<'a, T, ()>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}

impl<'a, T> RandomAccessIterator for Iter<'a, T> {
    #[inline]
    fn indexable(&self) -> usize {
        self.iter.indexable()
    }

    #[inline]
    fn idx(&mut self, index: usize) -> Option<&'a T> {
        self.iter.idx(index)
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}
