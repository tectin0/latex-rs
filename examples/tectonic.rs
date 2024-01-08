extern crate latex;
use latex::print;
use latex::{Document, Paragraph, Section};

fn main() {
    let mut doc = Document::new(latex::DocumentClass::Article);

    let mut section = Section::new("Section");

    let mut paragraph = Paragraph::new();
    paragraph.push("Hello, ");
    paragraph.push("world!");

    section.push(paragraph);

    doc.push(section);

    let rendered = print(&doc).unwrap();

    let pdf_data: Vec<u8> = tectonic::latex_to_pdf(rendered).expect("processing failed");
    println!("Output PDF size is {} bytes", pdf_data.len());

    std::fs::write("output.pdf", pdf_data).expect("could not write PDF");
}
