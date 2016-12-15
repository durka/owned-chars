#![deny(missing_docs)]

//! This crate provides two owned iterators over String: OwnedChars and OwnedCharIndices. They have
//! the same output as Chars and CharIndices, but creating the iterator consumes the String as
//! opposed to borrowing.

/// Extension trait for String providing owned char and char-index iterators
pub trait OwnedCharsExt {
    /// Gets an owning iterator over the chars (see `chars()`)
    fn into_chars(self) -> OwnedChars;
    /// Gets an owning iterator over the chars and their indices (see `char_indices()`)
    fn into_char_indices(self) -> OwnedCharIndices;
}

impl OwnedCharsExt for String {
    fn into_chars(self) -> OwnedChars {
        OwnedChars { s: self, i: 0 }
    }

    fn into_char_indices(self) -> OwnedCharIndices {
        OwnedCharIndices { s: self, i: 0 }
    }
}

/// Iterator over the chars of a string (the string is owned by the iterator)
#[derive(Clone, Debug)]
pub struct OwnedChars {
    s: String,
    i: usize,
}

/// Iterator over the chars of a string and their indices (the string is owned by the iterator)
#[derive(Clone, Debug)]
pub struct OwnedCharIndices {
    s: String,
    i: usize,
}

macro_rules! impls {
    ($s:ident) => {
        impl $s {
            /// Consume this struct and return the contained String
            pub fn into_inner(self) -> String {
                self.s
            }

            /// Borrow the contained String
            pub fn as_str(&self) -> &str {
                &self.s
            }
        }
    }
}
impls!(OwnedChars);
impls!(OwnedCharIndices);

impl Iterator for OwnedChars {
    type Item = char;
    
    fn next(&mut self) -> Option<char> {
        match unsafe { self.s.slice_unchecked(self.i, self.s.len()).chars().next() } {
            Some(c) => {
                self.i += c.len_utf8();
                Some(c)
            },
            None => None
        }
    }
}

impl Iterator for OwnedCharIndices {
    type Item = (usize, char);
    
    fn next(&mut self) -> Option<(usize, char)> {
        match unsafe { self.s.slice_unchecked(self.i, self.s.len()).chars().next() } {
            Some(c) => {
                let ret = Some((self.i, c));
                self.i += c.len_utf8();
                ret
            },
            None => None
        }
    }
}

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

