use super::node::*;
use std::cell::RefCell;
use std::clone::Clone;
use std::fmt::Debug;

pub struct ValidateProxy<T: Clone + Debug> {
    pub cherry: Cherry<T>,
    pub errors: RefCell<Vec<String>>,
}

impl<T: Clone + Debug> ValidateProxy<T> {
    pub fn collect(self) -> crate::node::Result<T> {
        if self.errors.borrow().is_empty() {
            Ok(self.cherry.to_owned())
        } else {
            Err(Error {
                label: self.cherry.name().to_owned(),
                msg: self.errors.into_inner(),
            })
        }
    }
}

pub trait Validate<T: Clone + Debug> {
    fn validate<IntoString, Predicate>(
        self,
        msg: IntoString,
        predicate: Predicate,
    ) -> ValidateProxy<T>
    where IntoString: Into<String>, Predicate: FnOnce(&T) -> bool;
}

impl<T: Clone + Debug> Validate<T> for Cherry<T> {
    fn validate<IntoString, Predicate>(
        self,
        msg: IntoString,
        predicate: Predicate,
    ) -> ValidateProxy<T>
        where IntoString: Into<String>, Predicate: FnOnce(&T) -> bool {
        if predicate(&self.quantity()) {
            ValidateProxy {
                cherry: self.to_owned(),
                errors: RefCell::new(vec![]),
            }
        }
        else {
            ValidateProxy {
                cherry: self.to_owned(),
                errors: RefCell::new(vec![msg.into()]),
            }
        }
    }
}

impl<T: Clone + Debug> Validate<T> for ValidateProxy<T> {
    fn validate<IntoString, Predicate>(
        self,
        msg: IntoString,
        predicate: Predicate,
    ) -> ValidateProxy<T>
        where IntoString: Into<String>, Predicate: FnOnce(&T) -> bool {
        if predicate(&self.cherry.quantity()) {
            self
        }
        else {
            self.errors.borrow_mut().push(msg.into());
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{Leaf, Error};
    use crate::validate::Validate;
    use uom::si::area::square_meter;
    use uom::si::f32::*;
    use uom::si::length::{meter};

    #[test]
    fn it_works() {
        let x = Leaf::new()
            .name("x")
            .value(Length::new::<meter>(2.0))
            .build();
        let y = Leaf::new()
            .name("y")
            .value(Length::new::<meter>(1.0))
            .build();
        let res = x * y;
        let validated = res
            .validate("must be less than 1.0!!", |quantity| quantity < &Area::new::<square_meter>(1.0))
            .collect();
        assert_eq!(Err(Error { label: "(mul)".to_string(), msg: vec!["must be less than 1.0!!".to_string()] }), validated);
    }
}
