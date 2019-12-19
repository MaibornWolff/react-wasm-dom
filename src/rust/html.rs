use std::collections::HashMap;
use std::fmt;

pub struct HTMLElement {
    pub tag: String,
    pub attributes: HashMap<&'static str, String>,
    pub children: Vec<HTMLValue>,
}

impl fmt::Display for HTMLElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}

pub enum HTMLValue {
    Element(HTMLElement),
    Text(String),
}

impl fmt::Display for HTMLValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Element(element) => write!(f, "{}", element),
            Self::Text(tag) => write!(f, "{}", tag),
        }
    }
}
