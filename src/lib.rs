#![deny(missing_docs)]

//! This crate provides two owned iterators over String: OwnedChars and OwnedCharIndices. They have
//! the same output as Chars and CharIndices, but creating the iterator consumes the String as
//! opposed to borrowing.

#[macro_use]
extern crate delegate;

use std::str::{Chars, CharIndices};
use std::iter::{Iterator, DoubleEndedIterator, FusedIterator};
use std::mem::{transmute, uninitialized};

/// Extension trait for String providing owned char and char-index iterators
pub trait OwnedCharsExt {
    /// Gets an owning iterator over the chars (see `chars()`)
    fn into_chars(self) -> OwnedChars;
    /// Gets an owning iterator over the chars and their indices (see `char_indices()`)
    fn into_char_indices(self) -> OwnedCharIndices;
}

impl OwnedCharsExt for String {
    fn into_chars(self) -> OwnedChars {
        OwnedChars::from_string(self)
    }

    fn into_char_indices(self) -> OwnedCharIndices {
        OwnedCharIndices::from_string(self)
    }
}

/// Iterator over the chars of a string (the string is owned by the iterator)
#[derive(Debug)]
pub struct OwnedChars {
    s: String,
    i: Chars<'static>,
}

/// Iterator over the chars of a string and their indices (the string is owned by the iterator)
#[derive(Debug)]
pub struct OwnedCharIndices {
    s: String,
    i: CharIndices<'static>,
}

macro_rules! impls {
    ($owned_struct:ident, $target_struct:ident, $method: ident, $item: ty) => {
        impl $owned_struct {
            /// Create Self from a String, moving the String into Self
            fn from_string(s: String) -> Self {
                unsafe {
                    // First, move the string
                    let mut owned = $owned_struct {
                        s: s,
                        i: uninitialized()
                    };

                    // Then, we can call .chars, which with have the same
                    // lifetime of the owner. We need the transmute to "widen"
                    // the lifetime into 'static which would allow us to store
                    // it in the owner.
                    owned.i = transmute::<$target_struct, $target_struct<'static>>(owned.s.$method());

                    owned
                }
            }

            /// Consume this struct and return the contained String
            pub fn into_inner(self) -> String {
                self.s
            }

            delegate! {
                target self.i {
                    /// Borrow the contained String
                    pub fn as_str(&self) -> &str;
                }
            }
        }

        impl Iterator for $owned_struct {
            type Item = $item;

            delegate! {
                target self.i {
                    fn next(&mut self) -> Option<$item>;
                    fn count(self) -> usize;
                    fn size_hint(&self) -> (usize, Option<usize>);
                    fn last(self) -> Option<$item>;
                }
            }
        }

        impl DoubleEndedIterator for $owned_struct {
            delegate! {
                target self.i {
                    fn next_back(&mut self) -> Option<$item>;
                }
            }
        }

        impl FusedIterator for $owned_struct {}
    };
}

impls!(OwnedChars, Chars, chars, char);
impls!(OwnedCharIndices, CharIndices, char_indices, (usize, char));

#[test]
fn chars() {
    let s = String::from("héllo");
    assert_eq!(s.chars().collect::<Vec<_>>(),
               s.into_chars().collect::<Vec<_>>());
}

#[test]
fn unicode() {
    let s = String::from("héllo");
    assert_eq!(Some('é'), s.clone().into_chars().skip(1).next());
    assert_eq!(Some('l'), s.clone().into_chars().skip(2).next());
}

#[test]
fn char_indices() {
    let s = String::from("héllo");
    assert_eq!(s.char_indices().collect::<Vec<_>>(),
               s.into_char_indices().collect::<Vec<_>>());
}

#[test]
fn methods() {
    let s = String::from("héllo");
    let oc = s.clone().into_chars();
    let oci = s.clone().into_char_indices();
    assert_eq!(&s, oc.as_str());
    assert_eq!(&s, oci.as_str());
    assert_eq!(s, oc.into_inner());
    assert_eq!(s, oci.into_inner());
}

