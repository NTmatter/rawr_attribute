# RAWR Attribute
The RAWR utility leverages attributes to reference code at particular points in
a codebase's history.

There is currently no validation, so all arguments will be allowed. This will be
rectified as the utility is stabilized.

```rust
extern crate rawr_attribute;
use rawr_attribute::rawr;

#[rawr(
    kind = "constant", identifier = "f_pi",
    path = "src/constants.h", revision = "1",
    notes = "This probably shouldn't change, but it would be good to know if \
    the upstream team makes changes to the simulator."
)]
const PI: f64 = 3.14159;

#[rawr(
    kind = "function", identifier = "do_foo",
    path = "src/headers/examples.h", revision = "abc123def"
)]
fn foo() {
    todo!("Reimplement function from original codebase")
}
```
