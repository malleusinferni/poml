use super::Poml;

pub fn from_str<'a>(input: &'a str) -> Result<Poml> {

}

struct Substr<'a> {
    line: usize,
    content: &'a str,
}

struct Header<'a> {
    depth: u32,
    name: Substr<'a>,
    attributes: Substr<'a>,
}

enum Token<'a> {
    Header(Header<'a>),
    Content(Substr<'a>),
    Directive(Header<'a>),
    Invalid(Substr<'a>),
}

