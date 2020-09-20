## 🐕 malamute 🐕

The key insight behind *malamute* is that efficient permutations operate on the set of indices rather than directly on the set of objects.

All permutations are represented as `Vec<usize>`. Permutation algorithms are included in `src/shuffle.rs` and implemented via implementations of Rust's `Iterator` trait.
```rust
impl Iterator for _ {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        ...
    }
}
```

After the permutation operations are applied, the resulting set of indices can be applied to the original `Vec<T>` via the `Transform` trait.
```rust
pub trait Transform {
    fn transform(&mut self) -> bool;
}
```