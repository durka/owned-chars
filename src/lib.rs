#![deny(missing_docs)]

//! This crate provides two owned iterators over String: OwnedChars and OwnedCharIndices. They have
//! the same output as Chars and CharIndices, but creating the iterator consumes the String as
//! opposed to borrowing.
//! 
//! Do you think this should be included in Rust proper? [Comment
//! here](https://github.com/durka/owned-chars/issues/5) if so!

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

/// structs
mod structs {
    use std::str::{Chars, CharIndices};
    use std::iter::{Iterator, DoubleEndedIterator, FusedIterator};
    use std::mem::transmute;

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
                pub fn from_string(s: String) -> Self {
                    unsafe {
                        // First, we can call .chars/.char_indices, whose result will have the same
                        // lifetime as the owner. We need the transmute to "widen" the lifetime into
                        // 'static which allows us to store it in the struct.
                        //
                        // The struct fields are private, so users can't observe this fake static
                        // lifetime. Code within this module must never destructure the struct
                        // because it risks losing track of the real lifetime!
                        let i = transmute::<$target_struct, $target_struct<'static>>(s.$method());

                        // Now, move the string (but not the string data!)
                        $owned_struct { s, i }
                    }
                }

                /// Consume this struct and return the contained String
                pub fn into_inner(self) -> String {
                    self.s
                }

                /// Returns a string slice of contained `String`.
                ///
                /// # Example
                ///
                /// ```rust
                /// # use owned_chars::{OwnedChars, OwnedCharsExt};
                /// let mut chars: OwnedChars = String::from("abc").into_chars();
                /// assert_eq!(chars.get_inner(), "abc");
                /// chars.next();
                /// assert_eq!(chars.get_inner(), "abc");
                /// chars.next();
                /// chars.next();
                /// assert_eq!(chars.get_inner(), "abc");
                /// ```
                pub fn get_inner(&self) -> &str {
                    &self.s
                }

                /// Borrow the contained String
                pub fn as_str(&self) -> &str {
                    self.i.as_str()
                }
            }

            impl Iterator for $owned_struct {
                type Item = $item;

                fn next(&mut self) -> Option<$item> {
                    self.i.next()
                }
                fn count(self) -> usize {
                    self.i.count()
                }
                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.i.size_hint()
                }
                fn last(self) -> Option<$item> {
                    self.i.last()
                }
            }

            impl DoubleEndedIterator for $owned_struct {
                fn next_back(&mut self) -> Option<$item> {
                    self.i.next_back()
                }
            }

            impl FusedIterator for $owned_struct {}
        };
    }

    impls!(OwnedChars, Chars, chars, char);
    impls!(OwnedCharIndices, CharIndices, char_indices, (usize, char));
}

pub use structs::*;

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

