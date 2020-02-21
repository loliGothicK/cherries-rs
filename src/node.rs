extern crate uom;
extern crate serde;
use std::fmt;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess, DeserializeOwned};
use regex::Regex;
use std::fmt::Debug;

///
/// Trait for active expression node.
///
pub trait Cherries {
    fn name(&self) -> &String;
    fn value(&self) -> std::result::Result<f32, String>;
    fn symbol(&self) -> String;
    fn to_json(&self) -> String;
}

///
/// Expression node.
///
#[derive(Clone, Debug)]
pub struct Cherry<T: Clone + Debug> {
    label: String,
    value: T,
    previous: Option<String>,
}

impl<T: Clone + Debug + PartialEq> PartialEq for Cherry<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.label == other.label)
            && (self.value == other.value)
            && (self.previous == other.previous)
    }
}

impl<T: Clone + Debug + Serialize> Serialize for Cherry<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Cherry", 3)?;
        state.serialize_field("label", &self.label)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("previous", &self.previous)?;
        state.end()
    }
}

#[derive(Clone, Debug)]
struct CherryVisitor<T: Clone + Debug> {
    value_type: std::marker::PhantomData<T>,
}

impl<'de, T: Clone + Debug + Deserialize<'de>> CherryVisitor<T> {
    fn new() -> Self {
        CherryVisitor { value_type: std::marker::PhantomData }
    }
}

impl<'de, T: Clone + Debug + Deserialize<'de>> serde::de::Visitor<'de> for CherryVisitor<T> {
    type Value = Cherry<T>;

    fn expecting(&self, _: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Cherry<T>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let label = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let value = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let previous = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        Ok(Cherry{label, value, previous})
    }

    fn visit_map<V>(self, mut map: V) -> Result<Cherry<T>, V::Error>
    where
        V: MapAccess<'de>,
    {
        enum Field { Label, Value, Previous };
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`label`, `value`, or `previous`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "label" => Ok(Field::Label),
                            "value" => Ok(Field::Value),
                            "previous" => Ok(Field::Previous),
                            _ => Err(de::Error::unknown_field(value, &["label", "value", "previous"])),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        let mut label = None;
        let mut value = None;
        let mut previous = None;
        while let Some(key) = map.next_key()? {
            match key {
                Field::Label => {
                    if label.is_some() {
                        return Err(de::Error::duplicate_field("label"));
                    }
                    label = Some(map.next_value()?);
                }
                Field::Value => {
                    if value.is_some() {
                        return Err(de::Error::duplicate_field("value"));
                    }
                    value = Some(map.next_value()?);
                }
                Field::Previous => {
                    if previous.is_some() {
                        return Err(de::Error::duplicate_field("previous"));
                    }
                    previous = Some(map.next_value()?);
                }
            }
        }
        let label = label.ok_or_else(|| de::Error::missing_field("label"))?;
        let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
        let previous = previous.ok_or_else(|| de::Error::missing_field("previous"))?;
        Ok(Cherry{label, value, previous})
    }
}

impl<'de, T: Clone + Debug + Deserialize<'de>> Deserialize<'de> for Cherry<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["label", "value", "previous"];
        let visitor: CherryVisitor<T> = CherryVisitor::new();
        deserializer.deserialize_struct("Duration", FIELDS, visitor)
    }
}

impl<T: Clone + Debug> Cherries for Cherry<T> {
    ///
    /// Returns reference of node name .
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    ///
    /// fn main() {
    ///     let node = Leaf::new().value(1).name("node").build();
    ///     assert_eq!(node.name(), &"node".to_string());
    /// }
    ///
    /// ```
    fn name(&self) -> &String {
        self.name()
    }
    ///
    /// Returns node value or error string.
    ///
    /// This method try to parse value from format string for uom support.
    /// There should be some other better way (help me, please!).
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    /// extern crate uom;
    /// use uom::si::{f32::*, length::meter};
    ///
    /// fn main() {
    ///     let node = Leaf::new().value(1).name("node").build();
    ///     assert_eq!(node.value(), Ok(1.0));
    ///     let node = Leaf::new().value(Length::new::<meter>(2.0)).name("node").build();
    ///     assert_eq!(node.value(), Ok(2.0));
    /// }
    ///
    /// ```
    fn value(&self) -> std::result::Result<f32, String> {
        let re = Regex::new(r#"^(.*?) .*$"#).unwrap();
        let formats = format!("{:?}", self.quantity()).to_owned();
        match formats.parse::<f32>() {
            Ok(value) => Ok(value),
            Err(_) => re.captures_iter(formats.clone().as_str()).last().map_or(
                Err(formats.clone()),
                |x| {
                    x.get(1).map_or(Err(formats.clone()), |x| {
                        x.as_str().parse::<f32>().map_err(|_| formats)
                    })
                },
            ),
        }
    }
    ///
    /// Returns units symbol.
    ///
    /// Returns node qunatity units symbol string (if has quantity) or `dimensionless`.
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    /// extern crate uom;
    /// use uom::si::{f32::*, length::meter};
    ///
    /// fn main() {
    ///     let node = Leaf::new().value(1).name("node").build();
    ///     assert_eq!(node.symbol(), "dimensionless".to_string());
    ///     let node = Leaf::new().value(Length::new::<meter>(2.0)).name("node").build();
    ///     assert_eq!(node.symbol(), "m^1".to_string());
    /// }
    ///
    /// ```
    fn symbol(&self) -> String {
        let re = Regex::new(r#".*? (.*)"#).unwrap();
        let formats = format!("{:?}", self.quantity()).to_owned();
        re.captures_iter(formats.clone().as_str())
            .last()
            .map(|x| {
                x.get(1)
                    .map(|x| x.as_str().to_string())
                    .unwrap_or_else(|| "dimensionless".to_string())
            })
            .unwrap_or_else(|| "dimensionless".to_string())
    }
    ///
    /// Returns expression log as json string.
    ///
    /// The json has `label (string)`, `value (number)`, `units (string)`, and `subexpr (array of object)`.
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    /// extern crate uom;
    /// use uom::si::{f32::*, length::meter};
    ///
    /// fn main() {
    ///     let x = Leaf::new().value(1.0).name("x").build();
    ///     let y = Leaf::new().value(Length::new::<meter>(2.0)).name("y").build();
    ///     let res = x * y;
    ///     assert_eq!(
    ///         res.to_json(),
    ///         "{\
    ///             \"label\":\"(mul)\",\
    ///             \"value\":2,\
    ///             \"unit\":\"m^1\",\
    ///             \"subexpr\":[\
    ///                 {\
    ///                     \"label\":\"x\",\
    ///                     \"value\":1,\
    ///                     \"unit\":\"dimensionless\"\
    ///                 },\
    ///                 {\
    ///                     \"label\":\"y\",\
    ///                     \"value\":2,\
    ///                     \"unit\":\"m^1\"\
    ///                 }\
    ///             ]\
    ///         }".to_string()
    ///     );
    /// }
    ///
    /// ```
    fn to_json(&self) -> String {
        match &self.previous {
            Some(prev) => {
                format!(
                    "{{\"label\":\"{label}\",\"value\":{value},\"unit\":\"{unit}\",\"subexpr\":[{subexpr}]}}",
                    label = self.label,
                    unit = self.symbol(),
                    value = self.value().unwrap(),
                    subexpr = prev)
            },
            None => {
                format!(
                    "{{\"label\":\"{label}\",\"value\":{value},\"unit\":\"{unit}\"}}",
                    label = self.label,
                    unit = self.symbol(),
                    value = self.value().unwrap()
                )
            }
        }
    }
}

impl<T: Clone + Debug> Cherry<T> {
    ///
    /// Returns reference of quantity which node has.
    ///
    /// Returns node qunatity (if has quantity) or value (if dimensionless).
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    /// extern crate uom;
    /// use uom::si::{f32::*, length::meter};
    ///
    /// fn main() {
    ///     let node = Leaf::new().value(1).name("node").build();
    ///     assert_eq!(node.quantity(), &1);
    ///     let node = Leaf::new().value(Length::new::<meter>(2.0)).name("y").build();
    ///     assert_eq!(node.quantity(), &Length::new::<meter>(2.0));
    /// }
    ///
    /// ```
    pub fn quantity(&self) -> &T {
        &self.value
    }
    ///
    /// Returns reference of node name .
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    ///
    /// fn main() {
    ///     let node = Leaf::new().value(1).name("node").build();
    ///     assert_eq!(node.name(), &"node".to_string());
    /// }
    ///
    /// ```
    pub fn name(&self) -> &String {
        &self.label
    }
    ///
    /// Returns node which renamed (and sonsuming self).
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    ///
    /// fn main() {
    ///     let node = Leaf::new().value(1).name("node").build();
    ///     assert_eq!(node.name(), &"node".to_string());
    ///     let node = node.labeled("renamed");
    ///     assert_eq!(node.name(), &"renamed".to_string());
    /// }
    ///
    /// ```
    pub fn labeled<S: Into<String>>(self, name: S) -> Cherry<T> {
        Cherry {
            label: name.into(),
            value: self.value,
            previous: self.previous,
        }
    }
    ///
    /// Maps a `Cherry<T>` to `Cherry<U>` by applying a function to a contained quantity.
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    /// extern crate uom;
    /// use uom::si::{f32::*, length::meter};
    ///
    /// fn main() {
    ///     let x = Leaf::new()
    ///         .name("x")
    ///         .value(Length::new::<meter>(2.1))
    ///         .build();
    ///     let res = x.map(|x| x.floor::<meter>()).labeled("floor");
    ///     assert_eq!(&Length::new::<meter>(2.0), res.quantity());
    /// }
    ///
    /// ```
    pub fn map<F: FnOnce(&T) -> U, U: Clone + Debug>(&self, f: F) -> Cherry<U> {
        Node::new()
            .name("(map)")
            .value(f(self.quantity()).to_owned())
            .prev(self.to_json().to_owned())
            .build()
    }
    ///
    /// Returns `Ok(&self)` if `predicate(self.quantity())` is true, otherwise returns `Err(&self)`.
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    /// extern crate uom;
    /// use uom::si::{f32::*, length::meter};
    ///
    /// fn main() {
    ///     let x = Leaf::new()
    ///         .name("x")
    ///         .value(Length::new::<meter>(2.1))
    ///         .build();
    ///     let res = x.is_satisfy_with(|x| x < &Length::new::<meter>(2.0));
    ///     assert_eq!(Err(&x), res);
    /// }
    ///
    /// ```
    pub fn is_satisfy_with<Predicate: FnOnce(&T) -> bool>(
        &self,
        predicate: Predicate,
    ) -> std::result::Result<&Self, &Self> {
        if predicate(&self.value) {
            Ok(self)
        } else {
            Err(self)
        }
    }
    ///
    /// Applies `self.quantity()` to given function `f` and returns its result.
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    /// extern crate uom;
    /// use uom::si::{f32::*, length::meter};
    ///
    /// fn main() {
    ///     let x = Leaf::new()
    ///         .name("x")
    ///         .value(Length::new::<meter>(2.1))
    ///         .build();
    ///     let res = x.with(|x| x < &Length::new::<meter>(2.0));
    ///     assert_eq!(res, false);
    /// }
    ///
    /// ```
    pub fn with<U, F: FnOnce(&T) -> U>(&self, f: F) -> U {
        f(&self.value)
    }
}

#[derive(Debug, Default)]
pub struct Leaf<NameType, ValueType> {
    label: NameType,
    value: ValueType,
}

///
/// Leaf node builder.
///
impl Leaf<(), ()> {
    ///
    /// Makes new leaf builder with empty filed.
    ///
    pub fn new() -> Self {
        Leaf {
            label: (),
            value: (),
        }
    }
}

impl<T: Clone + Debug> Leaf<String, T> {
    ///
    /// Makes `Cherry<T>` from `self.label`and `self.value`.
    ///
    /// # Examples
    /// ```
    /// extern crate cherries;
    /// use cherries::node::{Leaf, Cherries};
    ///
    /// fn main() {
    ///     let x = Leaf::new()
    ///         .name("x")
    ///         .value(2)
    ///         .build();
    ///     assert_eq!(x.quantity(), &2);
    ///     assert_eq!(x.name(), &"x".to_string());
    /// }
    ///
    /// ```
    pub fn build(self) -> Cherry<T> {
        Cherry {
            label: self.label,
            value: self.value,
            previous: None,
        }
    }
}

impl<NameType, ValueType> Leaf<NameType, ValueType> {
    ///
    /// Sets field `label`.
    ///
    pub fn name<S: Into<String>>(self, name: S) -> Leaf<String, ValueType> {
        Leaf {
            label: name.into(),
            value: self.value,
        }
    }
    ///
    /// Sets field `value`.
    ///
    pub fn value<T: Clone + Debug>(self, val: T) -> Leaf<NameType, T> {
        Leaf {
            label: self.label,
            value: val,
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct Node<NameType, ValueType, PrevType> {
    label: NameType,
    value: ValueType,
    previous: PrevType,
}

#[doc(hidden)]
impl Node<(), (), ()> {
    pub fn new() -> Self {
        Node {
            label: (),
            value: (),
            previous: (),
        }
    }
}

#[doc(hidden)]
impl<T: Clone + Debug> Node<String, T, String> {
    pub fn build(self) -> Cherry<T> {
        Cherry {
            label: self.label,
            value: self.value,
            previous: Some(self.previous),
        }
    }
}

#[doc(hidden)]
impl<NameType, ValueType, PrevType> Node<NameType, ValueType, PrevType> {
    pub fn name<S: Into<String>>(self, name: S) -> Node<String, ValueType, PrevType> {
        Node {
            label: name.into(),
            value: self.value,
            previous: self.previous,
        }
    }
    pub fn value<T: Clone + Debug>(self, val: T) -> Node<NameType, T, PrevType> {
        Node {
            label: self.label,
            value: val,
            previous: self.previous,
        }
    }
    pub fn prev<S: Into<String>>(self, prev: S) -> Node<NameType, ValueType, String> {
        Node {
            label: self.label,
            value: self.value,
            previous: prev.into(),
        }
    }
}
