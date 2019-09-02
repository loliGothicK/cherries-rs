extern crate uom;

use regex::Regex;
use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug)]
pub struct Error {
    pub label: String,
    pub msg: Vec<String>,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        (self.label == other.label) && (self.msg == other.msg)
    }
}

pub trait Cherries {
    fn name(&self) -> &String;
    fn value(&self) -> std::result::Result<f32, String>;
    fn symbol(&self) -> String;
    fn to_json(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct Cherry<T: Clone + Debug> {
    label: String,
    value: T,
    previous: Option<String>,
}

impl<T: Clone + Debug + PartialEq> PartialEq for Cherry<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.label == other.label) && (self.value == other.value) && (self.previous == other.previous)
    }
}

impl<T: Clone + Debug> Cherries for Cherry<T> {
    fn name(&self) -> &String {
        self.name()
    }
    fn value(&self) -> std::result::Result<f32, String> {
        let re = Regex::new(r#"^(.*?) .*$"#).unwrap();
        let format = format!("{:?}", self.quantity()).to_owned();
        match format.parse::<f32>() {
            Ok(value) => Ok(value),
            Err(_) => {
                re.captures_iter(format.clone().as_str())
                  .last()
                  .map_or(Err(format.clone()), |x| {
                      x.get(1).map_or(Err(format.clone()), |x| {
                          x.as_str().parse::<f32>().map_err(|_| format)
                      })
                  })
            }
        }
    }
    fn symbol(&self) -> String {
        let re = Regex::new(r#".*? (.*)"#).unwrap();
        let format = format!("{:?}", self.quantity()).to_owned();
        re.captures_iter(format.clone().as_str())
            .last()
            .map(|x| {
                x.get(1).map(|x| x.as_str().to_string()).unwrap_or("dimensionless".to_string())
            }).unwrap_or("dimensionless".to_string())
    }
    fn to_json(&self) -> String {
        match &self.previous {
            Some(prev) => {
                format!(
                    "{{\"label\": \"{label}\", \"value\": {value}, \"unit\": \"{unit}\", \"subexpr\": [{subexpr}]}}",
                    label = self.label,
                    unit = self.symbol(),
                    value = self.value().unwrap(),
                    subexpr = prev)
            },
            None => {
                format!(
                    "{{\"label\": \"{label}\", \"value\": {value}, \"unit\": \"{unit}\"}}",
                    label = self.label,
                    unit = self.symbol(),
                    value = self.value().unwrap()
                )
            }
        }
    }
}

pub type Result<T> = std::result::Result<Cherry<T>, Error>;

impl<T: Clone + Debug> Cherry<T> {
    pub fn quantity(&self) -> &T {
        &self.value
    }
    pub fn name(&self) -> &String {
        &self.label
    }
    pub fn label<S: Into<String>>(self, name: S) -> Cherry<T> {
        Cherry {
            label: name.into(),
            value: self.value,
            previous: self.previous,
        }
    }
    pub fn map<F: FnOnce(&T) -> U, U: Clone + Debug>(&self, f: F) -> Cherry<U> {
        Node::new()
            .name("(map)")
            .value(f(self.quantity()).to_owned())
            .prev(self.to_json().to_owned())
            .build()
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
    pub fn build(self) -> Cherry<T> {
        Cherry {
            label: self.label,
            value: self.value,
            previous: None,
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

#[derive(Debug, Default)]
pub struct Node<NameType, ValueType, PrevType> {
    label: NameType,
    value: ValueType,
    previous: PrevType,
}

impl Node<(), (), ()> {
    pub fn new() -> Self {
        Node {
            label: (),
            value: (),
            previous: (),
        }
    }
}

impl<T: Clone + Debug> Node<String, T, String> {
    pub fn build(self) -> Cherry<T> {
        Cherry {
            label: self.label,
            value: self.value,
            previous: Some(self.previous),
        }
    }
}

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
