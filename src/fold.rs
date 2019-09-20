use super::node::{Cherries, Cherry, Node};
use std::boxed::Box;
use std::fmt::Debug;
use std::ops::{Add, Mul};
use std::vec::Vec;

#[doc(hidden)]
pub struct FoldProxy<T> {
    pub value: T,
    pub items: Vec<Box<dyn Cherries>>,
}

#[doc(hidden)]
impl<T: Clone + Debug> FoldProxy<T> {
    pub fn into_expr(self) -> Cherry<T> {
        Node::new()
            .name("foldl".to_string())
            .value(self.value.clone())
            .prev(
                self.items
                    .iter()
                    .map(|x| x.to_json())
                    .collect::<Vec<_>>()
                    .join(",")
                    .to_owned(),
            )
            .build()
    }
}

#[doc(hidden)]
impl<T: 'static + Clone + Debug + std::cmp::PartialOrd> FoldProxy<T> {
    pub fn max(self, other: Cherry<T>) -> FoldProxy<T> {
        use std::cmp::Ordering;
        let mut ret = FoldProxy {
            value: match (&self.value).partial_cmp(other.quantity()) {
                Some(Ordering::Less) => other.quantity().clone(),
                Some(Ordering::Greater) => self.value.clone(),
                Some(Ordering::Equal) => self.value.clone(),
                None => {
                    panic!(
                        "cannot compare {:?} and {:?}.",
                        self.value,
                        other.quantity()
                    );
                }
            },
            items: self.items,
        };
        ret.items.push(Box::new(other));
        ret
    }
    pub fn min(self, other: Cherry<T>) -> FoldProxy<T> {
        use std::cmp::Ordering;
        let mut ret = FoldProxy {
            value: match (&self.value).partial_cmp(other.quantity()) {
                Some(Ordering::Less) => self.value.clone(),
                Some(Ordering::Greater) => other.quantity().clone(),
                Some(Ordering::Equal) => self.value.clone(),
                None => {
                    panic!(
                        "cannot compare {:?} and {:?}.",
                        self.value,
                        other.quantity()
                    );
                }
            },
            items: self.items,
        };
        ret.items.push(Box::new(other));
        ret
    }
}

#[doc(hidden)]
impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Add<Cherry<U>> for FoldProxy<T>
where
    T: Add<U>,
    <T as Add<U>>::Output: Clone + Debug,
{
    type Output = FoldProxy<<T as Add<U>>::Output>;

    fn add(self, other: Cherry<U>) -> FoldProxy<<T as Add<U>>::Output> {
        let mut ret = FoldProxy {
            value: self.value.clone() + other.quantity().clone(),
            items: self.items,
        };
        ret.items.push(Box::new(other));
        ret
    }
}

#[doc(hidden)]
impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Mul<Cherry<U>> for FoldProxy<T>
where
    T: Mul<U>,
    <T as Mul<U>>::Output: Clone + Debug,
{
    type Output = FoldProxy<<T as Mul<U>>::Output>;

    fn mul(self, other: Cherry<U>) -> FoldProxy<<T as Mul<U>>::Output> {
        let mut ret = FoldProxy {
            value: self.value.clone() * other.quantity().clone(),
            items: self.items,
        };
        ret.items.push(Box::new(other));
        ret
    }
}

///
/// Fold left with product all given expression.
///
/// The difference from normal multiplication is that all nodes are recorded in a single node sub-expression.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate cherries;
/// # use cherries::node::{Cherries, Leaf};
/// # use cherries::prod_all;
/// # fn main() {
///     let a = Leaf::new().value(2).name("a").build();
///     let b = Leaf::new().value(3).name("b").build();
///     let c = Leaf::new().value(4).name("c").build();
///     let d = Leaf::new().value(1).name("d").build();
///     let res = prod_all!(a, b, c, d);
///     assert_eq!(&24, res.quantity());
/// # }
/// ```
#[macro_export]
macro_rules! prod_all {
    ( $head:expr, $( $tail:expr ),* ) => {
        {
            let head = $head;
            ($crate::fold::FoldProxy { value: head.quantity().clone(), items: vec![Box::new(head)] }$( * $tail)*).into_expr()
        }
    };
}

///
/// Fold left with addition all given expression.
///
/// The difference from normal addition is that all nodes are recorded in a single node sub-expression.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate cherries;
/// # use cherries::node::{Cherries, Leaf};
/// # use cherries::sum_all;
/// # fn main() {
///     let a = Leaf::new().value(2).name("a").build();
///     let b = Leaf::new().value(3).name("b").build();
///     let c = Leaf::new().value(4).name("c").build();
///     let d = Leaf::new().value(1).name("d").build();
///     let res = sum_all!(a, b, c, d);
///     assert_eq!(&10, res.quantity());
/// # }
/// ```
#[macro_export]
macro_rules! sum_all {
    ( $head:expr, $( $tail:expr ),* ) => {
        {
            let head = $head;
            ($crate::fold::FoldProxy { value: head.quantity().clone(), items: vec![Box::new(head)] }$( + $tail)*).into_expr()
        }
    };
}

///
/// Fold left with `min` all given expression.
///
/// This marco uses `partial_cmp` inside the expanded codes.
/// Panics if and only if `partial_cmp` returns `None`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate cherries;
/// # use cherries::node::{Cherries, Leaf};
/// # fn main() {
///     let a = Leaf::new().value(2).name("a").build();
///     let b = Leaf::new().value(3).name("b").build();
///     let c = Leaf::new().value(4).name("c").build();
///     let d = Leaf::new().value(1).name("d").build();
///     let res = minimum!(a, b, c, d);
///     assert_eq!(&1, res.quantity());
/// # }
/// ```
#[macro_export]
macro_rules! minimum {
    ( $head:expr, $( $tail:expr ),* ) => {
        {
            let head = $head;
            ($crate::fold::FoldProxy { value: head.quantity().clone(), items: vec![Box::new(head)] }$(.min($tail))*).into_expr()
        }
    };
}

///
/// Fold left with `max` all given expression.
///
/// This marco uses `partial_cmp` inside the expanded codes.
/// Panics if and only if `partial_cmp` returns `None`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate cherries;
/// # use cherries::node::{Cherries, Leaf};
/// # use cherries::maximum;
/// # fn id<T>(x: T) -> T { x }
/// # fn main() {
///     let a = Leaf::new().value(2).name("a").build();
///     let b = Leaf::new().value(3).name("b").build();
///     let c = Leaf::new().value(4).name("c").build();
///     let d = Leaf::new().value(1).name("d").build();
///     let res = maximum!(id(a), b, c, d);
///     assert_eq!(&4, res.quantity());
/// # }
/// ```
#[macro_export]
macro_rules! maximum {
    ( $head:expr, $( $tail:expr ),* ) => {
        {
            let head = $head;
            ($crate::fold::FoldProxy { value: head.quantity().clone(), items: vec![Box::new(head)] }$(.max($tail))*).into_expr()
        }
    };
}
