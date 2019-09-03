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
impl<T: 'static + Clone + Debug + std::cmp::Ord> FoldProxy<T> {
    pub fn max(self, other: Cherry<T>) -> FoldProxy<T> {
        use std::cmp::max;
        let mut ret = FoldProxy {
            value: max(self.value, other.quantity().clone()),
            items: self.items,
        };
        ret.items.push(Box::new(other));
        ret
    }
    pub fn min(self, other: Cherry<T>) -> FoldProxy<T> {
        use std::cmp::min;
        let mut ret = FoldProxy {
            value: min(self.value, other.quantity().clone()),
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
///
#[macro_export]
macro_rules! prod_all {
    ( $head:expr, $( $tail:expr ),* ) => {
        (crate::fold::FoldProxy { value: ($head).quantity().clone(), items: vec![Box::new($head)] }$( * $tail)*).into_expr()
    };
}

#[macro_export]
macro_rules! sum_all {
    ( $head:expr, $( $tail:expr ),* ) => {
        (crate::fold::FoldProxy { value: ($head).quantity().clone(), items: vec![Box::new($head)] }$( + $tail)*).into_expr()
    };
}

// TODO: min
#[macro_export]
macro_rules! minimum {
    ( $head:expr, $( $tail:expr ),* ) => {
        (crate::fold::FoldProxy { value: ($head).quantity().clone(), items: vec![Box::new($head)] }$(.min($tail))*).into_expr()
    };
}

// TODO: max
#[macro_export]
macro_rules! maximum {
    ( $head:expr, $( $tail:expr ),* ) => {
        (crate::fold::FoldProxy { value: ($head).quantity().clone(), items: vec![Box::new($head)] }$(.max($tail))*).into_expr()
    };
}
