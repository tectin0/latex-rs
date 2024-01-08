extern crate latex;

use latex::{
    bibliography, cited, print, Citation, CitationType, Citations, Cite, Document, DocumentClass,
    Element, Section,
};

bibliography! {
    CITATION1 = Citation {
        key: "key1",
        author: "author1",
        citation_type: CitationType::Article,
        title: "title1",
        year: "year1",
    },
    CITATION2 = Citation {
        key: "key2",
        citation_type: CitationType::Article,
        title: "title2",
        author: "author2",
        year: "year2",
    }
}

fn create_document() -> Document {
    let mut doc = Document::new(DocumentClass::Article);

    // Set some metadata for the document
    doc.preamble.title("My Document with Citation");
    doc.preamble.author("Noah Nachtigall");

    doc.push(Element::TitlePage)
        .push(Element::ClearPage)
        .push(Element::TableOfContents)
        .push(Element::ClearPage);

    let mut section_1 = Section::new("Section 1");

    section_1
        .push("Here is some text which will be put in paragraph 1.")
        .push(cited!("And here is some more text for paragraph 2.", CITATION1).as_str())
        .push(
            cited!(
                "And here is some more text for paragraph 3.",
                Citations(&[CITATION2, CITATION1])
            )
            .as_str(),
        );

    doc.push(section_1);

    let mut section_2 = Section::new("Section 2");
    section_2.push("More text...");
    doc.push(section_2);

    doc
}

fn main() {
    let doc = create_document();
    let rendered = print(&doc).unwrap();
    println!("{}", rendered);
}
