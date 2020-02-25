use super::node::{Cherries, Cherry, Node};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

///
/// impl Add<Cherry<U>> for Cherry<T>
///
/// Provides a operation `lhs + rhs` (`lhs: Cherry<T>, rhs: Cherry<U>`), where `T: Add<U>`.
///
/// # Examples
///
/// ```
/// extern crate cherries;
/// use cherries::node::{Leaf, Node, Cherry};
///
/// let x = Leaf::new()
///     .name("x")
///     .value(1)
///     .build();
/// let y = Leaf::new()
///     .name("y")
///     .value(1)
///     .build();
/// let res = x + y;
/// assert_eq!(res.quantity(), &2);
/// ```
impl<T, U> Add<Cherry<U>> for Cherry<T>
where
    T: 'static + Clone + Debug + Add<U>,
    U: 'static + Clone + Debug,
    <T as Add<U>>::Output: Clone + Debug,
{
    type Output = Cherry<<T as Add<U>>::Output>;

    fn add(self, other: Cherry<U>) -> Cherry<<T as Add<U>>::Output> {
        Node::new()
            .name("(add)")
            .value(self.quantity().clone() + other.quantity().clone())
            .prev(vec![self.to_json(), other.to_json()].join(","))
            .build()
    }
}

///
/// impl Sub<Cherry<U>> for Cherry<T>
///
/// Provides a operation `lhs - rhs` (`lhs: Cherry<T>, rhs: Cherry<U>`), where `T: Sub<U>`.
///
/// ```
/// extern crate cherries;
/// use cherries::node::{Leaf, Node, Cherry};
///
/// let x = Leaf::new()
///     .name("x")
///     .value(1)
///     .build();
/// let y = Leaf::new()
///     .name("y")
///     .value(1)
///     .build();
/// let res = x - y;
/// assert_eq!(res.quantity(), &0);
/// ```
impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Sub<Cherry<U>> for Cherry<T>
where
    T: Sub<U>,
    <T as Sub<U>>::Output: Clone + Debug,
{
    type Output = Cherry<<T as Sub<U>>::Output>;

    fn sub(self, other: Cherry<U>) -> Cherry<<T as Sub<U>>::Output> {
        Node::new()
            .name("(sub)")
            .value(self.quantity().clone() - other.quantity().clone())
            .prev(vec![self.to_json(), other.to_json()].join(","))
            .build()
    }
}

///
/// impl Mul<Cherry<U>> for Cherry<T>
///
/// Provides a operation `lhs * rhs` (`lhs: Cherry<T>, rhs: Cherry<U>`), where `T: Mul<U>`.
///
/// ```
/// extern crate cherries;
/// use cherries::node::{Leaf, Node, Cherry};
///
/// let x = Leaf::new()
///     .name("x")
///     .value(2)
///     .build();
/// let y = Leaf::new()
///     .name("y")
///     .value(2)
///     .build();
/// let res = x * y;
/// assert_eq!(res.quantity(), &4);
/// ```
impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Mul<Cherry<U>> for Cherry<T>
where
    T: Mul<U>,
    <T as Mul<U>>::Output: Clone + Debug,
{
    type Output = Cherry<<T as Mul<U>>::Output>;

    fn mul(self, other: Cherry<U>) -> Cherry<<T as Mul<U>>::Output> {
        Node::new()
            .name("(mul)")
            .value(self.quantity().clone() * other.quantity().clone())
            .prev(vec![self.to_json(), other.to_json()].join(","))
            .build()
    }
}

///
/// impl Div<Cherry<U>> for Cherry<T>
///
/// Provides a operation `lhs / rhs` (`lhs: Cherry<T>, rhs: Cherry<U>`), where `T: Div<U>`.
///
/// ```
/// extern crate cherries;
/// use cherries::node::{Leaf, Node, Cherry};
///
/// let x = Leaf::new()
///     .name("x")
///     .value(4)
///     .build();
/// let y = Leaf::new()
///     .name("y")
///     .value(2)
///     .build();
/// let res = x / y;
/// assert_eq!(res.quantity(), &2);
/// ```
impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Div<Cherry<U>> for Cherry<T>
where
    T: Div<U>,
    <T as Div<U>>::Output: Clone + Debug,
{
    type Output = Cherry<<T as Div<U>>::Output>;

    fn div(self, other: Cherry<U>) -> Cherry<<T as Div<U>>::Output> {
        Node::new()
            .name("(div)")
            .value(self.quantity().clone() / other.quantity().clone())
            .prev(vec![self.to_json(), other.to_json()].join(","))
            .build()
    }
}
