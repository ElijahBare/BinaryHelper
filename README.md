# Binary Helper
This crate serves 2 purposes:

1: converting a binary number to a base 10 number

2: converting a base 10 number to a binary number


## Example
```rust
fn is_10_binary_check(x) -> String {
    let result = dec2binary(x,3);
    if result == "0001010"{
        "X is 10 (0001010 in binary)!".to_string()
    }
}
```
