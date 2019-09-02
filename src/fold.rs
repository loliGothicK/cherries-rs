use super::node::{Cherries, Cherry, Node};
use std::boxed::Box;
use std::fmt::Debug;
use std::ops::{Add, Mul};
use std::vec::Vec;

pub struct FoldProxy<T> {
    pub value: T,
    pub items: Vec<Box<dyn Cherries>>,
}

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

#[macro_export]
macro_rules! prod_all {
    ($head:expr, $($tail:expr),+) => {
        prod_all_impl!( crate::fold::FoldProxy { value: ($head).quantity().clone(), items: vec![Box::new($head)] }, $($tail), *).into_expr()
    };
}

#[macro_export]
macro_rules! prod_all_impl {
    ($last:expr) => { ($last) };
    ($first:expr, $second:expr) => { ($first + $second) };
    ($first:expr, $second:expr, $($tail:expr),+) => { ($first * $second) * prod_all_impl!($($tail),*) };
}

#[macro_export]
macro_rules! sum_all {
    ($head:expr, $($tail:expr),+) => {
        sum_all_impl!( crate::fold::FoldProxy { value: ($head).quantity().clone(), items: vec![Box::new($head)] }, $($tail), *).into_expr()
    };
}

#[macro_export]
macro_rules! sum_all_impl {
    ($last:expr) => { ($last) };
    ($first:expr, $second:expr) => { ($first + $second) };
    ($first:expr, $second:expr, $($tail:expr),+) => { ($first * $second) + product_impl!($($tail),*) };
}

// TODO: min

// TODO: max
