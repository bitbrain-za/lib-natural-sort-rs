# Natural Sort

Rust implementation of a natural sort order for string slices.

## Details

Splits a string into non-numeric and numeric parts. Compares non-numeric parts like normal strings.
Converts numeric parts to numbers and compares those.

Acts recursively and stops at the first inequality.

## Example

```rust
use natural_sort::natural_sort;
let mut list = vec!["z10a", "b23g", "z999", "z10", "x12z34", "x12z101", "z9", "z3", "z101", "z5"];
let expected = vec!["b23g", "x12z34", "x12z101", "z3", "z5", "z9", "z10", "z10a", "z101", "z999"];
natural_sort(&mut list);
assert_eq!(list, expected); 
```

