safe_unwrap
===========

The `safe_unwrap` macros allows unwrapping and annotating that the unwrap
will never fail.

An example:

```
#[macro_use]
extern crate safe_unwrap;

fn main() {
  let res = Some(42);

  // we know that unwrapping res will never fail, so it is safe to call unwrap
  let val = safe_unwrap!("is constant value", res);

  assert_eq!(val, 42);
}
```

In release builds, `safe_unwrap!(expr)` is equivalent to `expr.unwrap()`;
in debug builds, `expect()` will be called with a message indicating that
the assumed invariant has been violated.

The crate does not require `std`.
