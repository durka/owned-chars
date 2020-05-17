# owned-chars

[![Travis CI](https://travis-ci.org/durka/owned-chars.svg)](https://travis-ci.org/durka/owned-chars)

This crate provides an extension trait for String with two methods, `into_chars` and `into_char_indices`. These methods parallel `String::chars` and `String::char_indices`, but the iterators they create consume the String instead of borrowing it.

### Release notes

- *0.3.0*
  - Rewrite to use `delegate` crate
  - Fix/breaking change: `OwnedChars::as_str` works the same way as `std::Chars::as_str`

### Example

```rust
use owned_chars::OwnedChars;

fn main() {
    let mut chars = OwnedChars::from_string("0123456789ABCDEF".to_owned());
    let next_is_digit = |chars: &mut OwnedChars| chars.next().map_or(false, |c| c.is_numeric());

    assert!(next_is_digit(&mut chars)); // 0
    assert!(next_is_digit(&mut chars)); // 1
    assert!(next_is_digit(&mut chars)); // 2
    assert!(next_is_digit(&mut chars)); // 3
    assert!(next_is_digit(&mut chars)); // 4
    assert!(next_is_digit(&mut chars)); // 5
    assert!(next_is_digit(&mut chars)); // 6
    assert!(next_is_digit(&mut chars)); // 7
    assert!(next_is_digit(&mut chars)); // 8
    assert!(next_is_digit(&mut chars)); // 9

    assert!(!next_is_digit(&mut chars)); // A
    assert!(!next_is_digit(&mut chars)); // B
    assert!(!next_is_digit(&mut chars)); // C
    assert!(!next_is_digit(&mut chars)); // D
    assert!(!next_is_digit(&mut chars)); // E
    assert!(!next_is_digit(&mut chars)); // F
}
```
