# Cherries the expression logging library

Cherries is a crate that does expression logging as json structure.

## Usage
uom requires rustc 1.37.0 or later. Add this to your Cargo.toml:

```yaml
[dependencies]
cherries = "0.1.0"
```

### Labeling

You can label to leaf with builder or rename with label method.

```rust
extern crate cherries;
use cherries::node::{Leaf, Cherries};

fn main() {
    // labeling
    let node = Leaf::new().value(1).name("node").build();
    assert_eq!(node.name(), &"node".to_string());
    
    // renaming
    let node = node.label("renamed");
    assert_eq!(node.name(), &"renamed".to_string());
}
```

### Validation

```rust
extern crate cherries;
use cherries::node::{Leaf, Cherry, Cherries};
use cherries::validate::{Validate, ValidateChain, Error};

fn main() {
    let node = Leaf::new().value(2).name("node").build();
    let validated = node
        .validate("must be even", |v| v % 2 == 0)
        .validate("must be less than 4", |v| v < 4)
        .collect();

    assert_eq!(validated, Ok(Leaf::new().value(2).name("node").build()));

    let node = Leaf::new().value(1).name("node").build();
    let validated = node
        .validate("must be even", |v| v % 2 == 0)
        .validate("must be less than 4", |v| v < 4)
        .collect();

    assert_eq!(
        Err(Error {
            label: "node".to_string(),
            msg: vec![
                 "must be even".to_string(),
                 "must be less than 4".to_string()
            ]
        }),
        validated
    );
}
```

### Get json string

```rust
extern crate cherries;
use cherries::node::{Leaf, Cherry, Cherries};
use cherries::validate::{Validate, ValidateChain, Error};

fn main() {
    let a = Leaf::new().value(2).name("a").build();
    let b = Leaf::new().value(3).name("b").build();
    let c = Leaf::new().value(4).name("c").build();
    let d = Leaf::new().value(1).name("d").build();
    
    let e = a + b;
    let f = c - d;
    let res = e * f;
    println!("{}", res.to_json());
}
```

Output:

```json5
{
   "label":"(mul)",
   "value":15,
   "unit":"dimensionless",
   "subexpr":[
      {
         "label":"(add)",
         "value":5,
         "unit":"dimensionless",
         "subexpr":[
            {
               "label":"a",
               "value":2,
               "unit":"dimensionless"
            },
            {
               "label":"b",
               "value":3,
               "unit":"dimensionless"
            }
         ]
      },
      {
         "label":"(sub)",
         "value":3,
         "unit":"dimensionless",
         "subexpr":[
            {
               "label":"c",
               "value":4,
               "unit":"dimensionless"
            },
            {
               "label":"d",
               "value":1,
               "unit":"dimensionless"
            }
         ]
      }
   ]
}
```

### Mapping

For example, show you how to use cherries with uom crate (units of measurement).
Let's applying method `floor` with turbofish using `map`.

```rust
extern crate cherries;
use cherries::node::{Leaf, Cherry, Cherries};
use cherries::validate::{Validate, ValidateChain, Error};
use uom::si::f32::*;
use uom::si::length::{meter};

fn main() {
    let x = Leaf::new()
        .name("x")
        .value(Length::new::<meter>(2.1))
        .build();
    let res = x.map(|x| x.floor::<meter>()).label("floor");
    assert_eq!(&Length::new::<meter>(2.0), res.quantity());
    assert_eq!(&"floor".to_string(), res.name());
    println!("{}", res.to_json());
}
```

Output:

```json5
{
   "label":"floor",
   "value":2,
   "unit":"m^1",
   "subexpr":[
      {
         "label":"x",
         "value":2.1,
         "unit":"m^1"
      }
   ]
}
```

## License

Licensed under MIT LICENSE.