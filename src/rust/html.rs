use crate::{constants::*, react::escape_html};

use js_sys::{JsString, Reflect};
use wasm_bindgen::JsCast;

pub struct HTMLElement {
    pub tag: JsString,
    pub attributes: js_sys::Object,
    pub children: Vec<HTMLValue>,
}

impl HTMLElement {
    pub fn render(&self) -> JsString {
        EMPTY.with(|empty| {
            OPEN_TAG.with(|open_tag| {
                CLOSE_TAG.with(|close_tag| {
                    SELF_OPEN_TAG.with(|self_open_tag| {
                        SELF_CLOSE_TAG.with(|self_close_tag| {
                            SPACE.with(|space| {
                                ATTR_START.with(|attr_start| {
                                    ATTR_END.with(|attr_end| {
                                        let mut res = open_tag.concat(&self.tag);
                                        js_sys::Object::keys(&self.attributes)
                                            .iter()
                                            .map(|key| {
                                                let value =
                                                    Reflect::get(&self.attributes, &key).unwrap();
                                                [key, value]
                                            })
                                            .for_each(|[attr_key, attr_value]| {
                                                res = res
                                                    .concat(&space)
                                                    .concat(&attr_key)
                                                    .concat(&attr_start)
                                                    .concat(&escape_html(
                                                        &attr_value.unchecked_into(),
                                                    ))
                                                    .concat(&attr_end);
                                            });
                                        if self.is_self_closing() {
                                            res.concat(&self_close_tag)
                                        } else {
                                            let children = self
                                                .children
                                                .iter()
                                                .map(|child| child.render())
                                                .collect::<js_sys::Array>()
                                                .join(empty);
                                            res.concat(&close_tag)
                                                .concat(&children)
                                                .concat(&self_open_tag)
                                                .concat(&self.tag)
                                                .concat(&close_tag)
                                        }
                                    })
                                })
                            })
                        })
                    })
                })
            })
        })
    }

    fn is_self_closing(&self) -> bool {
        SELF_CLOSING.with(|self_closing| self_closing.includes(&self.tag, 0))
    }
}

pub enum HTMLValue {
    Element(HTMLElement),
    Text(JsString),
    Comment,
}

impl HTMLValue {
    pub fn render(&self) -> JsString {
        match self {
            Self::Element(element) => element.render(),
            Self::Text(text) => escape_html(text),
            Self::Comment => COMMENT.with(|comment| comment.clone()),
        }
    }
}
