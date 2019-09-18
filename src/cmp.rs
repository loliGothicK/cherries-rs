use super::node::Cherry;
use std::cmp::Ordering;
use std::fmt::Debug;

///
/// impl PartialOrd for Cherry<T>
///
/// Implementation of `lhs.partial_cmp(&rhs)` (`lhs: &Cherry<T>, rhs: &Cherry<T>`), where `T: PartialOrd`.
/// Provides comparison operators for `Cherry<T>`.
///
/// # Examples
///
/// ```
/// extern crate cherries;
/// extern crate uom;
/// use cherries::node::{Leaf, Node};
/// use uom::si::f32::*;
/// use uom::si::length::meter;
/// use std::cmp::Ordering;
/// fn main() {
///    let x = Leaf::new().value(Length::new::<meter>(2.0)).name("x").build();
///    let y = Leaf::new().value(Length::new::<meter>(2.1)).name("y").build();
///    assert_eq!(x.partial_cmp(&y), Some(Ordering::Less));
///    assert_eq!(y.partial_cmp(&x), Some(Ordering::Greater));
///    assert_eq!(x.partial_cmp(&x), Some(Ordering::Equal));
///    assert_eq!(x < y, true);
///    assert_eq!(y < x, false);
///    assert_eq!(x > y, false);
///    assert_eq!(y > x, true);
///    assert_eq!(x == x, true);
/// }
/// ```
impl<T> PartialOrd for Cherry<T>
where
    T: 'static + Clone + Debug + PartialOrd,
{
    fn partial_cmp(&self, other: &Cherry<T>) -> Option<Ordering> {
        self.quantity().partial_cmp(other.quantity())
    }
}
