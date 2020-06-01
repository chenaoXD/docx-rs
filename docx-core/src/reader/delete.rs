use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Delete {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut runs: Vec<Run> = vec![];
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    if let XMLElement::Run = e {
                        runs.push(Run::read(r, attrs)?);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    let run = if !runs.is_empty() {
                        std::mem::replace(&mut runs[0], Run::new())
                    } else {
                        Run::new()
                    };
                    if e == XMLElement::Delete {
                        let mut del = Delete::new(run);
                        for attr in attrs {
                            let local_name = &attr.name.local_name;
                            if local_name == "author" {
                                del = del.author(&attr.value);
                            } else if local_name == "date" {
                                del = del.date(&attr.value);
                            }
                        }
                        if runs.len() > 1 {
                            for r in runs.into_iter().skip(1) {
                                del = del.add_run(r);
                            }
                        }
                        return Ok(del);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
