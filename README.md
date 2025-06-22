# RAWR Macros
The RAWR utility uses marker macros to reference code at particular points in
a codebase's history.

There is currently no validation, so all arguments will be allowed. This will be
rectified as the utility is stabilized.

```rust
extern crate rawr_attribute;
use rawr_macro::{rawr, Rawr, rawr_fn};

#[rawr(
    kind = "constant",
    ident = "f_pi",
    path = "src/constants.h",
    rev = "123abc456",
    notes = "This probably shouldn't change, but it would be good to know if \
    the upstream team makes changes to the simulator."
)]
const PI: f64 = 3.14159;

#[rawr(
    kind = "function",
    ident = "do_foo",
    path = "src/headers/examples.h",
    rev = "abc123def"
)]
fn foo() {
    todo!("Reimplement function from original codebase")
}
```
