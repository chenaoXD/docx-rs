use docx_rs::*;

use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./fixtures/paragraph/paragraph.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();

    let docx = read_docx(&buf).unwrap();

    for sec in docx.document.children {
        match sec {
            DocumentChild::Paragraph(para) => {
                // println!("{:#?}", para)
                println!("{}", para.text());
            }
            _ => {}
        }
    }
}
