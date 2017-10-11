extern crate serde;

pub mod encode;
//pub mod decode;

pub struct Tag {
    pub name: String,
    pub attrs: String,
    pub content: String,
    pub children: Vec<Tag>,
}

pub enum Poml {
    V1(Vec<Tag>),
}
