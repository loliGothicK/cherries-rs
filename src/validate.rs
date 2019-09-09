use super::node::*;
use std::cell::RefCell;
use std::clone::Clone;
use std::fmt::Debug;

/// For validation.
///
#[derive(Debug)]
pub struct Error {
    pub label: String,
    pub msg: Vec<String>,
    pub tree: String,
}

/// Type synonym for `std::result::Result<Cherry<T>, Error>`.
///
/// Used in validation.
///
pub type Result<T> = std::result::Result<Cherry<T>, Error>;

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        (self.label == other.label) && (self.msg == other.msg)
    }
}

pub struct ValidateChain<T: Clone + Debug> {
    pub cherry: Cherry<T>,
    pub errors: RefCell<Vec<String>>,
}

///
/// Immediate proxy for validation
///
/// Provides method `into_result` to aggregate validation error.
///
impl<T: Clone + Debug> ValidateChain<T> {
    ///
    /// Aggregates validation error.
    ///
    /// Coverts `ValidateProxy<T>` to [`cherries::Result<T>`](../node/type.Result.html).
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate cherries;
    /// use cherries::{node::Leaf, validate::{Validate, Error}};
    /// extern crate uom;
    /// use uom::si::{f32::*, length::meter, area::square_meter};
    ///
    /// fn main() {
    ///    let x = Leaf::new()
    ///        .name("x")
    ///        .value(Length::new::<meter>(2.0))
    ///        .build();
    ///    let y = Leaf::new()
    ///        .name("y")
    ///        .value(Length::new::<meter>(1.0))
    ///        .build();
    ///    let res = x * y;
    ///    let validated = res
    ///        .validate("must be less than 1.0!!", |quantity| {
    ///            quantity < &Area::new::<square_meter>(1.0)
    ///        })
    ///        .validate("must be less than 0.0!!", |quantity| {
    ///            quantity < &Area::new::<square_meter>(0.0)
    ///        })
    ///        .into_result();
    ///    assert_eq!(
    ///        Err(Error {
    ///            label: "(mul)".to_string(),
    ///            msg: vec![
    ///                 "must be less than 1.0!!".to_string(),
    ///                 "must be less than 0.0!!".to_string()
    ///            ],
    ///            tree: "json tree".to_string(),
    ///        }),
    ///        validated
    ///    );
    /// }
    /// ```
    pub fn into_result(self) -> Result<T> {
        if self.errors.borrow().is_empty() {
            Ok(self.cherry.to_owned())
        } else {
            Err(Error {
                label: self.cherry.name().to_owned(),
                msg: self.errors.into_inner(),
                tree: self.cherry.to_json(),
            })
        }
    }
}

///
/// Trait: Validate
///
/// Provides method `validate`.
///
pub trait Validate<T: Clone + Debug> {
    fn validate<IntoString, Predicate>(
        self,
        msg: IntoString,
        predicate: Predicate,
    ) -> ValidateChain<T>
    where
        IntoString: Into<String>,
        Predicate: FnOnce(&T) -> bool;
}

///
/// Trait: Validate for `Cherry<T>`
///
/// Provides `validate` function.
/// `self.validate(predicate)` returns `ValidateProxy<T>`.
/// `ValidateProxy<T>` also has `validate` to chain for validation.
///
/// # Examples
///
/// ```
/// extern crate cherries;
/// use cherries::{node::Leaf, validate::{Validate, Error}};
/// extern crate uom;
/// use uom::si::{f32::*, length::meter, area::square_meter};
///
/// fn main() {
///    let x = Leaf::new()
///        .name("x")
///        .value(Length::new::<meter>(2.0))
///        .build();
///    let y = Leaf::new()
///        .name("y")
///        .value(Length::new::<meter>(1.0))
///        .build();
///    let res = x * y;
///    let validated = res
///        .validate("must be less than 1.0!!", |quantity| {
///            quantity < &Area::new::<square_meter>(1.0)
///        })
///        .into_result();
///    assert_eq!(
///        Err(Error {
///            label: "(mul)".to_string(),
///            msg: vec!["must be less than 1.0!!".to_string()],
///            tree: "json tree".to_string(),
///        }),
///        validated
///    );
/// }
/// ```
impl<T: Clone + Debug> Validate<T> for Cherry<T> {
    fn validate<IntoString, Predicate>(
        self,
        msg: IntoString,
        predicate: Predicate,
    ) -> ValidateChain<T>
    where
        IntoString: Into<String>,
        Predicate: FnOnce(&T) -> bool,
    {
        if predicate(&self.quantity()) {
            ValidateChain {
                cherry: self.to_owned(),
                errors: RefCell::new(vec![]),
            }
        } else {
            ValidateChain {
                cherry: self.to_owned(),
                errors: RefCell::new(vec![msg.into()]),
            }
        }
    }
}

///
/// For validation chaining.
///
/// `self.validate(predicate)` returns `ValidateProxy<T>`.
///
/// # Examples
///
/// ```
/// extern crate cherries;
/// use cherries::{node::Leaf, validate::{Validate, Error}};
/// extern crate uom;
/// use uom::si::{f32::*, length::meter, area::square_meter};
///
/// fn main() {
///    let x = Leaf::new()
///        .name("x")
///        .value(Length::new::<meter>(2.0))
///        .build();
///    let y = Leaf::new()
///        .name("y")
///        .value(Length::new::<meter>(1.0))
///        .build();
///    let res = x * y;
///    let validated = res
///        .validate("must be less than 1.0!!", |quantity| {
///            quantity < &Area::new::<square_meter>(1.0)
///        })
///        .validate("must be less than 0.0!!", |quantity| {
///            quantity < &Area::new::<square_meter>(0.0)
///        })
///        .into_result();
///    assert_eq!(
///        Err(Error {
///            label: "(mul)".to_string(),
///            msg: vec![
///                 "must be less than 1.0!!".to_string(),
///                 "must be less than 0.0!!".to_string()
///            ],
///            tree: "json tree".to_string(),
///        }),
///        validated
///    );
/// }
/// ```
impl<T: Clone + Debug> Validate<T> for ValidateChain<T> {
    fn validate<IntoString, Predicate>(
        self,
        msg: IntoString,
        predicate: Predicate,
    ) -> ValidateChain<T>
    where
        IntoString: Into<String>,
        Predicate: FnOnce(&T) -> bool,
    {
        if predicate(&self.cherry.quantity()) {
            self
        } else {
            self.errors.borrow_mut().push(msg.into());
            self
        }
    }
}
