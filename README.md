# bloomin

A bloom filter is a probabalistic data structure that can be used to check wheather a value is in a given set.
It provides a very space efficient way to quickly check for membership.

## Installation

To use this Bloom filter implementation in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
bloomin= "0.1.0"
```
## Usage

The bloom filter can be initialised by calling the constructor with the false positive rate and number of expected elements.

Functions implemented:
- Insert: Insert into the bloom filter
- Contains: Checks if a value is present in the set (could return false positives)
- Clear: Resets the contents of the bloom filter

```rust
let mut filter = BloomFilter::new(0.01, 100);
filter.insert(&10);
assert!(filter.contains(&10));
assert!(!filter.contains(&20));
filter.clear();

assert!(!filter.contains(&10));
```

## Performance Considerations

The Bloom filter provides a space-efficient way to test for membership, but there is a small probability of false positives. This can be controlled by explicitly mentioning a small value or increasing the number of hash fns and filter size which would impact performance and space, however.

It's important to choose appropriate filter size and hash function parameters based on the expected number of elements and acceptable false positive rate for your specific use case.
