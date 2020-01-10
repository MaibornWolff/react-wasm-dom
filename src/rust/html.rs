use std::collections::HashMap;
use std::fmt;

pub struct HTMLElement {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<HTMLValue>,
}

impl HTMLElement {
    pub fn is_self_closing(&self) -> bool {
        match self.tag.as_ref() {
            "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta"
            | "param" | "source" | "track" | "wbr" => true,
            _ => false,
        }
    }
}

impl fmt::Display for HTMLElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}", self.tag)?;
        for (attr_key, attr_value) in self.attributes.iter() {
            write!(f, " {}=\"{}\"", attr_key, attr_value)?;
        }
        if self.is_self_closing() {
            write!(f, "/>")?;
        } else {
            write!(
                f,
                ">{}</{}>",
                self.children
                    .iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(""),
                self.tag
            )?;
        }
        Ok(())
    }
}

pub enum HTMLValue {
    Element(HTMLElement),
    Text(String),
    Comment,
}

impl fmt::Display for HTMLValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Element(element) => write!(f, "{}", element),
            Self::Text(text) => {
                let escaped_text = text
                    .replace("&", "&amp;")
                    .replace("\"", "&quot;")
                    .replace("'", "&#x27;")
                    .replace(">", "&gt;")
                    .replace("<", "&lt;");
                write!(f, "{}", escaped_text)?;
                Ok(())
            }
            Self::Comment => write!(f, "<!-- -->"),
        }
    }
}
