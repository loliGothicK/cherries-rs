extern crate uom;

use regex::Regex;
use std::boxed::Box;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};
use std::vec::Vec;

#[derive(Debug)]
pub struct ErrInfo {
    label: String,
    msg: Vec<String>,
}

pub trait ExprNode {
    fn name(&self) -> &String;
    fn value(&self) -> std::result::Result<f32, String>;
    fn symbol(&self) -> std::result::Result<String, String>;
}

pub struct Expr<T: Clone + Debug> {
    label: String,
    value: T,
    previous: Vec<Box<dyn ExprNode>>,
}

impl<T: Clone + Debug> ExprNode for Expr<T> {
    fn name(&self) -> &String {
        self.name()
    }
    fn value(&self) -> std::result::Result<f32, String> {
        let re = Regex::new(r#"^(.*?) .*$"#).unwrap();
        let format = format!("{:?}", self.quantity()).to_owned();
        re.captures_iter(format.clone().as_str())
            .last()
            .map_or(Err(format.clone()), |x| {
                x.get(1).map_or(Err(format.clone()), |x| {
                    x.as_str().parse::<f32>().map_err(|_| format)
                })
            })
    }
    fn symbol(&self) -> std::result::Result<String, String> {
        let re = Regex::new(r#".*? (.*)"#).unwrap();
        let format = format!("{:?}", self.quantity()).to_owned();
        re.captures_iter(format.clone().as_str())
            .last()
            .map_or(Err(format.clone()), |x| {
                x.get(1).map(|x| x.as_str().to_string()).ok_or(format)
            })
    }
}

pub type Result<T> = std::result::Result<Expr<T>, ErrInfo>;

impl<T: Clone + Debug> Expr<T> {
    pub fn quantity(&self) -> &T {
        &self.value
    }
    pub fn name(&self) -> &String {
        &self.label
    }
    fn label<S: Into<String>>(self, name: S) -> Expr<T> {
        Expr {
            label: name.into(),
            value: self.value,
            previous: self.previous,
        }
    }
}

#[derive(Debug, Default)]
pub struct Leaf<NameType, ValueType> {
    label: NameType,
    value: ValueType,
}

impl Leaf<(), ()> {
    pub fn new() -> Self {
        Leaf {
            label: (),
            value: (),
        }
    }
}

impl<T: Clone + Debug> Leaf<String, T> {
    pub fn build(self) -> Expr<T> {
        Expr {
            label: self.label,
            value: self.value,
            previous: vec![],
        }
    }
}

impl<NameType, ValueType> Leaf<NameType, ValueType> {
    pub fn name<S: Into<String>>(self, name: S) -> Leaf<String, ValueType> {
        Leaf {
            label: name.into(),
            value: self.value,
        }
    }
    pub fn value<T: Clone + Debug>(self, val: T) -> Leaf<NameType, T> {
        Leaf {
            label: self.label,
            value: val,
        }
    }
}

impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Add<Expr<U>> for Expr<T>
where
    T: Add<U>,
    <T as Add<U>>::Output: Clone + Debug,
{
    type Output = Expr<<T as Add<U>>::Output>;

    fn add(self, other: Expr<U>) -> Expr<<T as Add<U>>::Output> {
        Expr {
            label: "(+)".to_string(),
            value: self.quantity().clone() + other.quantity().clone(),
            previous: vec![Box::new(self), Box::new(other)],
        }
    }
}

impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Sub<Expr<U>> for Expr<T>
where
    T: Sub<U>,
    <T as Sub<U>>::Output: Clone + Debug,
{
    type Output = Expr<<T as Sub<U>>::Output>;

    fn sub(self, other: Expr<U>) -> Expr<<T as Sub<U>>::Output> {
        Expr {
            label: "(-)".to_string(),
            value: self.quantity().clone() - other.quantity().clone(),
            previous: vec![Box::new(self), Box::new(other)],
        }
    }
}

impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Mul<Expr<U>> for Expr<T>
where
    T: Mul<U>,
    <T as Mul<U>>::Output: Clone + Debug,
{
    type Output = Expr<<T as Mul<U>>::Output>;

    fn mul(self, other: Expr<U>) -> Expr<<T as Mul<U>>::Output> {
        Expr {
            label: "(*)".to_string(),
            value: self.quantity().clone() * other.quantity().clone(),
            previous: vec![Box::new(self), Box::new(other)],
        }
    }
}

impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Div<Expr<U>> for Expr<T>
where
    T: Div<U>,
    <T as Div<U>>::Output: Clone + Debug,
{
    type Output = Expr<<T as Div<U>>::Output>;

    fn div(self, other: Expr<U>) -> Expr<<T as Div<U>>::Output> {
        Expr {
            label: "(*)".to_string(),
            value: self.quantity().clone() / other.quantity().clone(),
            previous: vec![Box::new(self), Box::new(other)],
        }
    }
}

struct FoldProxy<T> {
    value: T,
    items: Vec<Box<dyn ExprNode>>,
}

impl<T: Clone + Debug> FoldProxy<T> {
    fn into_expr(self) -> Expr<T> {
        Expr {
            label: "product".to_string(),
            value: self.value.clone(),
            previous: self.items,
        }
    }
}

impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Mul<Expr<U>> for FoldProxy<T>
where
    T: Mul<U>,
    <T as Mul<U>>::Output: Clone + Debug,
{
    type Output = FoldProxy<<T as Mul<U>>::Output>;

    fn mul(self, other: Expr<U>) -> FoldProxy<<T as Mul<U>>::Output> {
        let mut ret = FoldProxy {
            value: self.value.clone() * other.value.clone(),
            items: self.items,
        };
        ret.items.push(Box::new(other));
        ret
    }
}

#[macro_export]
macro_rules! product {
    ($head:expr, $($tail:expr),+) => {
        product_impl!( FoldProxy { value: ($head).quantity().clone(), items: vec![Box::new($head)] }, $($tail), *).into_expr()
    };
}

#[macro_export]
macro_rules! product_impl {
    ($last:expr) => { ($last) };
    ($first:expr, $second:expr) => { ($first + $second) };
    ($first:expr, $second:expr, $($tail:expr),+) => { ($first * $second) * product_impl!($($tail),*) };
}

#[cfg(test)]
mod tests {
    #[macro_use]
    use crate::node::{Leaf, ExprNode, FoldProxy};
    use uom::si::area::square_millimeter;
    use uom::si::f32::*;
    use uom::si::length::{meter, millimeter};
    use uom::si::volume::cubic_meter;

    #[test]
    fn it_works() {
        let x = Leaf::new()
            .name("x")
            .value(Length::new::<millimeter>(2.0))
            .build();
        let y = Leaf::new()
            .name("y")
            .value(Length::new::<millimeter>(1.0) * 2.0)
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

        let res = product!(x, y, z);
        assert_eq!(res.quantity(), &Volume::new::<cubic_meter>(64.0));
        for node in res.previous {
            println!("{:?}", node.value());
        }
    }
}
