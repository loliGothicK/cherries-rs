extern crate uom;

pub mod cmp;
pub mod node;
pub mod ops;
#[macro_use]
pub mod fold;
pub mod validate;

#[cfg(test)]
mod tests {
    use crate::node::{Cherries, Leaf};
    use uom::si::area::{square_meter, square_millimeter};
    use uom::si::f32::*;
    use uom::si::length::{meter, millimeter};

    #[test]
    fn basic_tests() {
        let x = Leaf::new()
            .name("x")
            .value(uom::si::f32::Length::new::<millimeter>(2.0))
            .build();
        let y = Leaf::new()
            .name("y")
            .value(uom::si::f32::Length::new::<millimeter>(1.0) * 2.0)
            .build();
        assert_eq!(x.quantity(), &Length::new::<millimeter>(2.0));
        assert_eq!(y.quantity(), &Length::new::<millimeter>(2.0));
        assert_eq!(x.symbol(), "m^1".to_string());
        assert_eq!((x + y).quantity().value, 0.004);
        let x = Leaf::new()
            .name("x")
            .value(Length::new::<millimeter>(2.0))
            .build();
        let y = Leaf::new()
            .name("y")
            .value(Length::new::<millimeter>(4.0))
            .build();
        let res = x * y;
        assert_eq!(res.symbol(), "m^2".to_string());
        assert_eq!(res.quantity(), &Area::new::<square_millimeter>(8.0));

        let x = Leaf::new().name("x").value(2.0).build();
        let y = Leaf::new()
            .name("y")
            .value(Length::new::<meter>(4.0))
            .build();
        let z = Leaf::new()
            .name("z")
            .value(Length::new::<meter>(8.0))
            .build();

        let res = prod_all!(x, y, z).labeled("xyz");
        assert_eq!(&Area::new::<square_meter>(64.0), res.quantity());
        assert_eq!(res.name(), &"xyz".to_string());
        println!("{}", res.to_json());
    }
    #[test]
    fn map_tests() {
        let x = Leaf::new()
            .name("x")
            .value(Length::new::<meter>(2.1))
            .build();
        let res = x.map(|x| x.floor::<meter>()).labeled("floor");
        assert_eq!(&Length::new::<meter>(2.0), res.quantity());
        assert_eq!(&"floor".to_string(), res.name());
        println!("{}", res.to_json());
    }
}
#[cfg(test)]
mod label_tests {
    use crate::node::{Cherries, Leaf};
    #[test]
    fn it_works() {
        // labeling
        let node = Leaf::new().value(1).name("node").build();
        assert_eq!(node.name(), &"node".to_string());

        // renaming
        let node = node.labeled("renamed");
        assert_eq!(node.name(), &"renamed".to_string());
        let a = Leaf::new().value(2).name("a").build();
        let b = Leaf::new().value(3).name("b").build();
        let c = Leaf::new().value(4).name("c").build();
        let d = Leaf::new().value(1).name("d").build();

        let e = a + b;
        let f = c - d;
        let res = e * f;
        println!("{}", res.to_json());
    }
}
#[cfg(test)]
mod validate_tests {
    use crate::node::{Cherries, Cherry, Leaf};
    use crate::validate::{Error, Validate, ValidateChain};
    #[test]
    fn it_works() {
        let node = Leaf::new().value(2).name("node").build();
        let validated = node.validate("must be even", |v| v % 2 == 0).into_result();

        assert_eq!(validated, Ok(Leaf::new().value(2).name("node").build()));
    }
}
#[cfg(test)]
mod fold_tests {
    use crate::node::{Cherries, Leaf};
    use uom::si::i32::*;
    use uom::si::length::meter;
    #[test]
    fn it_works() {
        let a = Leaf::new().value(Length::new::<meter>(2)).name("a").build();
        let b = Leaf::new().value(Length::new::<meter>(3)).name("b").build();
        let c = Leaf::new().value(Length::new::<meter>(4)).name("c").build();
        let d = Leaf::new().value(Length::new::<meter>(1)).name("d").build();
        let res = maximum!(a, b, c, d);
        assert_eq!(&Length::new::<meter>(4), res.quantity());
        println!("{}", res.to_json());
    }
}
