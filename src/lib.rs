#![deny(missing_docs)]

//! This crate provides two owned iterators over String: [OwnedChars] and [OwnedCharIndices]. They have
//! the same output as [Chars] and [CharIndices], respectively, but creating the iterator consumes the String as
//! opposed to borrowing.
//! 
//! Do you think this should be included in Rust proper? [Comment
//! here](https://github.com/durka/owned-chars/issues/5) if so!
//! 
//! [Chars]: std::str::Chars
//! [CharIndices]: std::str::CharIndices

#[macro_use]
extern crate delegate_attr;

/// Extension trait for String providing owned char and char-index iterators.
pub trait OwnedCharsExt {
    /// Returns an owning iterator over the [`char`]s of a string.
    ///
    /// It is an owning alternative to [`str::chars`] method.
    fn into_chars(self) -> OwnedChars;

    /// Returns an owning iterator over the [`char`]s of a string, and their positions.
    ///
    /// It is an owning alternative to [`str::char_indices`] method.
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

mod structs {
    use std::str::{Chars, CharIndices};
    use std::iter::{Iterator, DoubleEndedIterator, FusedIterator};
    use std::mem::transmute;

    /// Iterator over the chars of a string (the string is owned by the iterator).
    #[derive(Debug)]
    pub struct OwnedChars {
        s: String,
        i: Chars<'static>,
    }

    /// Iterator over the chars of a string and their indices (the string is owned by the iterator).
    #[derive(Debug)]
    pub struct OwnedCharIndices {
        s: String,
        i: CharIndices<'static>,
    }

    macro_rules! impls {
        ($owned_struct:ident, $target_struct:ident, $method: ident, $item: ty) => {
            impl $owned_struct {
                #[doc = concat!(
                    "Creates new `",
                    stringify!($owned_struct),
                    "` from the String.",
                )]
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

                /// Consume this struct and return the contained String.
                pub fn into_inner(self) -> String {
                    self.s
                }

                #[delegate(self.i)]
                /// Views the underlying data as a subslice of the original data.
                /// 
                /// # Example
                ///
                /// ```rust
                /// # use owned_chars::{OwnedChars, OwnedCharsExt};
                /// let mut chars: OwnedChars = String::from("abc").into_chars();
                /// assert_eq!(chars.as_str(), "abc");
                /// chars.next();
                /// assert_eq!(chars.as_str(), "bc");
                /// chars.next();
                /// chars.next();
                /// assert_eq!(chars.as_str(), "");
                /// ```
                pub fn as_str(&self) -> &str;
            }

            #[delegate(self.i)]
            impl Iterator for $owned_struct {
                type Item = $item;

                fn next(&mut self) -> Option<$item>;
                fn count(self) -> usize;
                fn size_hint(&self) -> (usize, Option<usize>);
                fn last(self) -> Option<$item>;
            }

            #[delegate(self.i)]
            impl DoubleEndedIterator for $owned_struct {
                fn next_back(&mut self) -> Option<$item>;
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

