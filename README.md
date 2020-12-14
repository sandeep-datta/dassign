# DASSIGN: Destructuring assignment

Use this crate for simple tuple destructuring assignments while the compiler implementation of this feature is being stabilized.

```rust
let mut x;
let mut y;

for _ in 1..100 {
    dassign!((x, y) = func_returning_tuple());
}
```

Once this feature is stabilized simply remove the `dassign!` macro from your code.

## Reference
- RFC: https://github.com/rust-lang/rfcs/pull/2909.
- Tracking issue: https://github.com/rust-lang/rust/issues/71126
