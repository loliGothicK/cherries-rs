extern crate uom;
use std::vec::Vec;
use std::ops::{Add, Sub, Mul, Div};
use std::boxed::Box;
use regex::Regex;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ErrInfo {
    label: String,
    msg: Vec<String>,
}

pub trait ExprNode {
    fn label(&self) -> &String;
    fn value(&self) -> std::result::Result<f32, String>;
    fn symbol(&self) -> std::result::Result<String, String>;
}

pub struct Expr<T: Clone + Debug> {
    label: String,
    value: T,
    previous: Vec<Box<dyn ExprNode>>,
}

impl<T: Clone + Debug> ExprNode for Expr<T> {
    fn label(&self) -> &String {
        self.label()
    }
    fn value(&self) -> std::result::Result<f32, String> {
        let re = Regex::new(r#"^(.*?) .*$"#).unwrap();
        let format = format!("{:?}", self.quantity()).as_str().to_owned();
        re.captures_iter(format.clone().as_str())
            .last()
            .map_or(
                Err(format.clone()),
                |x|x.get(1).map_or(
                    Err(format.clone()),
                    |x|x.as_str().parse::<f32>().map_err(|_| format)
                )
            )
    }
    fn symbol(&self) -> std::result::Result<String, String> {
        let re = Regex::new(r#".*? (.*)"#).unwrap();
        let format = format!("{:?}", self.quantity()).as_str().to_owned();
        re.captures_iter(format.clone().as_str())
            .last()
            .map_or(
                Err(format.clone()),
                |x| x.get(1).map(|x|x.as_str().to_string()).ok_or(format)
        )
    }
}

pub type Result<T> = std::result::Result<Expr<T>, ErrInfo>;

impl<T: Clone + Debug> Expr<T> {
    pub fn quantity(&self) -> &T {
        &self.value
    }
    pub fn label(&self) -> &String {
        &self.label
    }
}

#[derive(Debug, Default)]
pub struct LeafBuilder<NameType, ValueType> {
    label: NameType,
    value: ValueType,
}

impl LeafBuilder<(), ()> {
    pub fn new() -> Self {
        LeafBuilder {
            label: (),
            value: (),
        }
    }
}

impl<T: Clone + Debug> LeafBuilder<String, T> {
    pub fn build(self) -> Expr<T> {
        Expr {
            label: self.label,
            value: self.value,
            previous: vec![],
        }
    }
}

impl<NameType, ValueType> LeafBuilder<NameType, ValueType> {
    pub fn name<S: Into<String>>(self, name: S) -> LeafBuilder<String, ValueType> {
        LeafBuilder {
            label: name.into(),
            value: self.value,
        }
    }
    pub fn value<T: Clone + Debug>(self, val: T) -> LeafBuilder<NameType, T> {
        LeafBuilder {
            label: self.label,
            value: val,
        }
    }
}

impl<T: 'static + Clone + Debug> Add for Expr<T>
    where T: Add<Output=T> + Clone {
    type Output = Expr<T>;

    fn add(self, other: Expr<T>) -> Expr<T> {
        Expr {
            label: "(+)".to_string(),
            value: self.quantity().clone() + other.quantity().clone(),
            previous: vec![
                Box::new(self),
                Box::new(other),
            ],
        }
    }
}

impl<T: 'static + Clone + Debug> Sub for Expr<T>
    where T: Sub<Output=T> + Clone {
    type Output = Expr<T>;

    fn sub(self, other: Expr<T>) -> Expr<T> {
        Expr {
            label: "(-)".to_string(),
            value: self.quantity().clone() - other.quantity().clone(),
            previous: vec![
                Box::new(self),
                Box::new(other),
            ],
        }
    }
}

impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Mul<Expr<U>> for Expr<T>
    where T: Mul<U>,
          <T as Mul<U>>::Output: Clone + Debug {
    type Output = Expr<<T as Mul<U>>::Output>;

    fn mul(self, other: Expr<U>) -> Expr<<T as Mul<U>>::Output> {
        Expr {
            label: "(*)".to_string(),
            value: self.quantity().clone() * other.quantity().clone(),
            previous: vec![
                Box::new(self),
                Box::new(other),
            ],
        }
    }
}

impl<T: 'static + Clone + Debug, U: 'static + Clone + Debug> Div<Expr<U>> for Expr<T>
    where T: Div<U>,
          <T as Div<U>>::Output: Clone + Debug {
    type Output = Expr<<T as Div<U>>::Output>;

    fn div(self, other: Expr<U>) -> Expr<<T as Div<U>>::Output> {
        Expr {
            label: "(*)".to_string(),
            value: self.quantity().clone() / other.quantity().clone(),
            previous: vec![
                Box::new(self),
                Box::new(other),
            ],
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
    where T: Mul<U>,
          <T as Mul<U>>::Output: Clone + Debug {
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
    use crate::node::{LeafBuilder, ExprNode, FoldProxy};
    use uom::si::f32::*;
    use uom::si::length::{millimeter, meter};
    use uom::si::area::square_millimeter;
    use uom::si::volume::cubic_meter;
    #[test]
    fn it_works() {
        let x = LeafBuilder::new().name("x").value(Length::new::<millimeter>(2.0)).build();
        let y = LeafBuilder::new().name("y").value(Length::new::<millimeter>(2.0)).build();
        assert_eq!(x.quantity(), &Length::new::<millimeter>(2.0));
        assert_eq!(y.quantity(), &Length::new::<millimeter>(2.0));
        assert_eq!(x.symbol(), Ok("m^1".to_string()));
        assert_eq!((x+y).quantity().value, 0.004);
        let x = LeafBuilder::new().name("x").value(Length::new::<millimeter>(2.0)).build();
        let y = LeafBuilder::new().name("y").value(Length::new::<millimeter>(4.0)).build();
        let res = x * y;
        assert_eq!(res.symbol(), Ok("m^2".to_string()));
        assert_eq!(res.quantity(), &Area::new::<square_millimeter>(8.0));

        let x = LeafBuilder::new().name("x").value(Length::new::<meter>(2.0)).build();
        let y = LeafBuilder::new().name("y").value(Length::new::<meter>(4.0)).build();
        let z = LeafBuilder::new().name("z").value(Length::new::<meter>(8.0)).build();

        let res = product!(x,y,z);
        assert_eq!(res.quantity(), &Volume::new::<cubic_meter>(64.0));
        for node in res.previous {
            println!("{:?}", node.value());
        }
    }
}
