use super::node::{Cherries, Cherry, Node};
use std::ops::{Add, Div, Mul, Sub};
use std::fmt::Debug;

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
