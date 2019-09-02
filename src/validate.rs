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
    fn validate<Predicate: FnOnce(&T) -> std::result::Result<(), String>>(
        self,
        predicate: Predicate,
    ) -> ValidateProxy<T>;
}

impl<T: Clone + Debug> Validate<T> for Cherry<T> {
    fn validate<Predicate: FnOnce(&T) -> std::result::Result<(), String>>(
        self,
        predicate: Predicate,
    ) -> ValidateProxy<T> {
        match predicate(&self.quantity()) {
            Ok(()) => ValidateProxy {
                cherry: self.to_owned(),
                errors: RefCell::new(vec![]),
            },
            Err(msg) => ValidateProxy {
                cherry: self.to_owned(),
                errors: RefCell::new(vec![msg.to_owned()]),
            },
        }
    }
}

impl<T: Clone + Debug> Validate<T> for ValidateProxy<T> {
    fn validate<Predicate: FnOnce(&T) -> std::result::Result<(), String>>(
        self,
        predicate: Predicate,
    ) -> ValidateProxy<T> {
        match predicate(self.cherry.quantity()) {
            Ok(()) => self,
            Err(msg) => {
                self.errors.borrow_mut().push(msg.to_owned());
                self
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{Cherries, Cherry, Leaf};
    use crate::validate::Validate;
    use uom::si::area::square_meter;
    use uom::si::f32::*;
    use uom::si::length::{meter, millimeter};

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
            .validate(|quantity| {
                if quantity < &Area::new::<square_meter>(1.0) {
                    Ok(())
                } else {
                    Err("greater than 1.0!!".to_string())
                }
            })
            .collect();
        println!("{:?}", validated);
    }
}
