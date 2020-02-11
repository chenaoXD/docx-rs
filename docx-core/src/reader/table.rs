use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::types::*;

impl ElementReader for Table {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut t = Table::new(vec![]);
        let mut grid_col: Vec<usize> = vec![];
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::TableRow => {
                            t = t.add_row(TableRow::read(r, &attributes)?);
                            continue;
                        }
                        XMLElement::TableWidth => {
                            let (w, width_type) = read_width(&attributes)?;
                            t = t.width(w, width_type);
                            continue;
                        }
                        XMLElement::Justification => {
                            t = t.align(TableAlignmentType::from_str(&attributes[0].value)?);
                        }
                        XMLElement::TableIndent => {
                            let (w, _) = read_width(&attributes)?;
                            t = t.indent(w);
                            continue;
                        }
                        XMLElement::TableBorders => {
                            // TODO: Support later
                        }
                        XMLElement::TableCellMargin => {
                            // TODO: Support later
                        }
                        XMLElement::GridCol => {
                            let (w, _) = read_width(&attributes)?;
                            grid_col.push(w);
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Table {
                        t = t.set_grid(grid_col);
                        return Ok(t);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_table_with_width_prop() {
        let c =
            r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:tbl>
    <w:tblPr>
        <w:tblW w:w="9638" w:type="dxa"/>
    </w:tblPr>
    <w:tblGrid>
        <w:gridCol w:w="3212"/>
        <w:gridCol w:w="3213"/>
        <w:gridCol w:w="3213"/>
    </w:tblGrid>
</w:tbl>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let t = Table::read(&mut parser, &[]).unwrap();
        assert_eq!(
            t,
            Table::new(vec![])
                .set_grid(vec![3212, 3213, 3213])
                .width(9638, WidthType::DXA)
        );
    }

    #[test]
    fn test_read_table_with_layout() {
        let c =
            r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:tbl>
    <w:tblPr>
        <w:jc w:val="center"/>
        <w:tblInd w:w="100" w:type="dxa"/>
    </w:tblPr>
</w:tbl>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let t = Table::read(&mut parser, &[]).unwrap();
        assert_eq!(
            t,
            Table::new(vec![])
                .align(TableAlignmentType::Center)
                .indent(100)
        );
    }
}
