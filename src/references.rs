/// Trait to implement for a citation
pub trait Cite {
    /// Returns a string representation of the citation
    fn cite(&self) -> String;
}

/// Citation struct for use in a CitationEnum
pub struct Citation {
    /// Key for citation reference
    pub key: &'static str,
    /// Type
    pub citation_type: CitationType,
    /// Title
    pub title: &'static str,
    /// Author
    pub author: &'static str,
    /// Year
    pub year: &'static str,
}

impl Citation {
    /// Returns a string representation of the citation in BibTeX format
    pub fn to_bib_entry(&self) -> String {
        match self.citation_type {
            CitationType::Article => {
                format!(
                    "@article{{{key},\nauthor={{{author}}},\ntitle={{{title}}},\nyear={{{year}}}\n}}",
                    key = self.key,
                    author = self.author,
                    title = self.title,
                    year = self.year
                )
            }
        }
    }
}

impl Cite for Citation {
    fn cite(&self) -> String {
        format!("\\cite{{{}}}", self.key)
    }
}

/// Type of Citation
pub enum CitationType {
    /// Article
    Article,
}

/// Multiple citations
pub struct Citations<'a>(pub &'a [Citation]);

impl<'a> From<&'a [Citation]> for Citations<'a> {
    fn from(citations: &'a [Citation]) -> Self {
        Self(citations)
    }
}

impl Cite for Citations<'_> {
    fn cite(&self) -> String {
        let mut citations = String::new();
        for citation in self.0 {
            citations.push_str(&citation.cite());
        }
        citations
    }
}

/// Example
///
/// ```
/// use latex::cited;
/// let cited_string = cited!(
/// "This is a cited example", CitationEnum::Citation1,
/// "This is another cited example", CitationEnum::Citation2
/// );
/// ```
#[macro_export]
macro_rules! cited {
    ($($text:expr, $citation:expr),*) => {
        {
            let mut cited_string = String::new();
            $(
                cited_string.push_str($text);
                cited_string.push_str(&*($citation.cite()));
            )*
            cited_string
        }
    };
}

/// Bibliography struct for use in a bibliography macro
pub struct Bibliography(pub &'static [Citation]);

impl Bibliography {
    /// Writes the bibliography to a file
    pub fn write_to_bib_file(&self, file_name: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(file_name)?;
        file.write_all(<&Bibliography as Into<String>>::into(self).as_bytes())?;
        Ok(())
    }

    /// Returns a string representation of the bibliography in BibTeX format
    pub fn to_filecontents(&self) -> String {
        let mut filecontents = String::new();

        filecontents.push_str("\\begin{filecontents*}{main.bib}\n");

        for citation in self.0 {
            filecontents.push_str(&citation.to_bib_entry());
            filecontents.push('\n');
        }

        filecontents.push_str("\\end{filecontents*}\n");

        filecontents
    }
}

impl Into<String> for &Bibliography {
    fn into(self) -> String {
        let mut bibliography = String::new();

        for citation in self.0 {
            bibliography.push_str(&citation.to_bib_entry());
            bibliography.push('\n');
        }

        bibliography
    }
}

impl ToString for Bibliography {
    fn to_string(&self) -> String {
        <&Bibliography as Into<String>>::into(self)
    }
}

/// Constructs a constant citation for each citation in the bibliography
/// and also stores them in a constant vector
///
/// Example
///
/// ```
/// use latex::bibliography;
/// let bibliography = bibliography! {
///    citation1 = Citation {
///       key: "key1",
///       author: "author1",
///       year: "year1",
///    },
///    citation2 = Citation {
///        key: "key2",
///        author: "author2",
///        year: "year2",
///    },
/// };
///
/// fn main () {
///   println!("{}", citation1);
///   println!("{}", citation2);
/// }
/// ```
#[macro_export]
macro_rules! bibliography {
    ($($name:ident = $citation:expr),*) => {
        $(
            pub const $name: Citation = $citation;
        )*

        const LEN: usize = [$($name),*].len();

        pub const BIBLIOGRAPHY: latex::Bibliography = latex::Bibliography(&[$($name),*]);
    };
}
