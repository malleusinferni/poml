use super::{Poml, Tag};

impl Poml {
    pub fn to_string(&self) -> Result<String> {
        match self {
            &Poml::V1(ref tags) => {
                let mut encoder = EncoderV1::new();
                encoder.encode_tags(tags.as_slice())?;
                Ok(encoder.buf)
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidTagName(String),
    InvalidAttrs(String),
    InvalidContent(String),
}

pub type Result<T, E=Error> = ::std::result::Result<T, E>;

struct EncoderV1 {
    depth: u32,
    buf: String,
}

impl EncoderV1 {
    fn new() -> Self {
        let buf = "#!poml v1\n".to_owned();
        let depth = 0;
        EncoderV1 { buf, depth }
    }

    fn encode_tags(&mut self, tags: &[Tag]) -> Result<()> {
        let current_depth = self.depth;

        for tag in tags {
            self.depth = current_depth + 1;

            for _ in 0 .. self.depth { self.buf.push('#'); }

            verify_ident(&tag.name)?;
            verify_attrs(&tag.attrs)?;

            self.buf.push_str(&tag.name);
            self.buf.push(' ');
            self.buf.push_str(&tag.attrs);
            self.buf.push('\n');

            for line in tag.content.lines() {
                if line.starts_with('#') {
                    return Err(Error::InvalidContent(line.to_owned()));
                } else {
                    self.buf.push_str(line);
                    self.buf.push('\n');
                }
            }

            self.encode_tags(&tag.children)?;
        }

        Ok(())
    }
}

fn verify_ident(name: &str) -> Result<()> {
    let fail = || Err(Error::InvalidTagName(name.to_owned()));

    match name.chars().next() {
        Some(ch) if ch.is_alphabetic() => (),
        _ => fail()?,
    }

    for ch in name.chars().skip(1) {
        if ch.is_alphabetic() { continue; }
        if ch.is_digit(10) { continue; }
        if ch == '_' { continue; }
        fail()?;
    }

    Ok(())
}

fn verify_attrs(attrs: &str) -> Result<()> {
    if attrs.contains('\n') || attrs.contains('\r') {
        Err(Error::InvalidAttrs(attrs.to_owned()))
    } else {
        Ok(())
    }
}

#[test]
fn examples() {
    let doc = Poml::V1({
        vec![
            Tag {
                name: "BODY".to_owned(),
                attrs: "X=1 Y=2".to_owned(),
                content: "".to_owned(),
                children: vec![],
            },
        ]
    });

    let expected = "#!poml v1\n#BODY X=1 Y=2\n";
    assert_eq!(expected, doc.to_string().unwrap());
}
