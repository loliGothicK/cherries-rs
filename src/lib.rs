extern crate uom;

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
    use uom::si::volume::cubic_meter;

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

        let res = prod_all!(x, y, z).label("xyz");
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
        let res = x.map(|x| x.floor::<meter>()).label("floor");
        assert_eq!(&Length::new::<meter>(2.0), res.quantity());
        assert_eq!(&"floor".to_string(), res.name());
        println!("{}", res.to_json());
    }
}
