# Shigens

The `shigens` is a math library for calculate quaternion.

When there was such a variable

```text
x = 1 + 2i + 3j + 4k
```

in `shigens` can write like this.

```rust
use shigens::*;

let x = quaternion(1, 2, 3, 4);
// or
let x = r(1) + i(2) + j(3) + k(4);
```

# Examples

So far, I have implemented only basic operations.

```rust
use shigens::*;

assert_eq!(i(1) * i(1), r(-1));
assert_eq!(j(1) * j(1), r(-1));
assert_eq!(k(1) * k(1), r(-1));
```

```rust
use shigens::*;

let x1 = quaternion(2, 3, 4, 5);
let x2 = quaternion(6, 7, 8, 9);
let y = x1 + x2;

assert_eq!(y.r.get_value(), 2 + 6);
assert_eq!(y.i.get_value(), 3 + 7);
assert_eq!(y.j.get_value(), 4 + 8);
assert_eq!(y.k.get_value(), 5 + 9);
```

```rust
use shigens::*;

let x1 = quaternion(2, 3, 4, 5);
let x2 = quaternion(6, 7, 8, 9);
let y = x1 * x2;

assert_eq!(y.r.get_value(), 2 * 6 - 3 * 7 - 4 * 8 - 5 * 9);
assert_eq!(y.i.get_value(), 2 * 7 + 3 * 6 + 4 * 9 - 5 * 8);
assert_eq!(y.j.get_value(), 2 * 8 + 4 * 6 + 5 * 7 - 3 * 9);
assert_eq!(y.k.get_value(), 2 * 9 + 5 * 6 + 3 * 8 - 4 * 7);
```

```rust
use shigens::*;

let x = quaternion(2, 3, 4, 5);
let y = x * i(2);

assert_eq!(y.r.get_value(), 3 * 2 * -1);
assert_eq!(y.i.get_value(), 2 * 2);
assert_eq!(y.j.get_value(), 5 * 2);
assert_eq!(y.k.get_value(), 4 * 2 * -1);
```
