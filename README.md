# owned-chars

[![Travis CI](https://travis-ci.org/durka/owned-chars.svg)](https://travis-ci.org/durka/owned-chars)

This crate provides an extension trait for String with two methods, `into_chars` and `into_char_indices`. These methods parallel `String::chars` and `String::char_indices`, but the iterators they create consume the String instead of borrowing it.

