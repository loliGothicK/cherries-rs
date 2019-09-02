extern crate uom;

pub mod node;
pub mod ops;
#[macro_use]
pub mod fold;

#[cfg(test)]
mod tests {
    use crate::node::{Cherries, Leaf};
    use uom::si::area::square_millimeter;
    use uom::si::f32::*;
    use uom::si::length::{meter, millimeter};
    use uom::si::volume::cubic_meter;

    #[test]
    fn it_works() {
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
        assert_eq!(x.symbol(), Ok("m^1".to_string()));
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
        assert_eq!(res.symbol(), Ok("m^2".to_string()));
        assert_eq!(res.quantity(), &Area::new::<square_millimeter>(8.0));

        let x = Leaf::new()
            .name("x")
            .value(Length::new::<meter>(2.0))
            .build();
        let y = Leaf::new()
            .name("y")
            .value(Length::new::<meter>(4.0))
            .build();
        let z = Leaf::new()
            .name("z")
            .value(Length::new::<meter>(8.0))
            .build();

        let res = prod_all!(x, y, z).label("xyz");
        assert_eq!(&Volume::new::<cubic_meter>(64.0), res.quantity());
        assert_eq!(res.name(), &"xyz".to_string());
        println!("{}", res.to_json());
    }
}
