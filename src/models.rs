//! The models crate handles instantiating and using documents and indexes.

/// Document contains information pertaining to a document such as a webpage.
/// Documents have many indexes.
pub struct Document {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub content: String,
}

impl Document {
    /// new creates a new instantiation of a Document.
    ///
    /// # Examples
    ///
    /// new Document
    ///
    /// ```
    /// let d = Document::new(
    ///     1,
    ///     "http://acme.com/about".to_string(),
    ///     "About Us".to_string(),
    ///     "We are Acme and we make things.".to_string(),
    /// );
    pub fn new(id: i32, url: String, title: String, content: String) -> Document {
        Document {
            id: id,
            url: url,
            title: title,
            content: content,
        }
    }
}

/// Index contains information about a particular word in a document. A document
/// can only contain unique indexes. The locations field should hold all
/// occurences of the word in the document and their location in the document.
pub struct Index {
    pub id: i32,
    pub word: String,
    pub locations: Vec<i32>,
}

impl Index {
    /// new creates a new instantiation of an Index.
    ///
    /// # Examples
    ///
    /// new Index
    ///
    /// ```
    /// let i = Index::new(
    ///     1,
    ///     "acme".to_string(),
    ///     vec![3, 15, 72],
    /// );
    pub fn new(id: i32, word: String, locations: Vec<i32>) -> Index {
        Index {
            id: id,
            word: word,
            locations: locations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_document() {
        let d = Document::new(
            1,
            "http://acme.com/about".to_string(),
            "About Us".to_string(),
            "We are Acme and we make things.".to_string(),
        );

        assert_eq!(1, d.id);
        assert_eq!("http://acme.com/about", d.url);
        assert_eq!("About Us", d.title);
        assert_eq!("We are Acme and we make things.", d.content);
    }

    #[test]
    fn test_new_index() {
        let i = Index::new(
            1,
            "acme".to_string(),
            vec![3, 15, 72],
        );

        assert_eq!(1, i.id);
        assert_eq!("acme", i.word);
        assert_eq!(3, i.locations.len());
    }
}