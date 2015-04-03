//! The models crate handles instantiating and using documents and indexes.

/// Document contains information pertaining to a document such as a webpage.
/// Documents have many indexes.
pub struct Document {
    id: i32,
    url: String,
    title: String,
    content: String,
}

impl Document {
    /// new creates a new instantiation of a Document.
    ///
    /// # Examples
    ///
    /// new Document
    ///
    /// ```
    /// use kensaku::models::Document;
    ///
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

    /// id() is a getter for the private field id
    ///
    /// # Examples
    ///
    /// Get id
    ///
    /// ```
    /// use kensaku::models::Document;
    ///
    /// let d = Document::new(
    ///     1,
    ///     "http://acme.com/about".to_string(),
    ///     "About Us".to_string(),
    ///     "We are Acme and we make things.".to_string(),
    /// );
    /// let id: i32 = d.id();
    /// ```
    pub fn id(&self) -> i32 {
        return self.id;
    }

    /// url() is a getter for the private field url
    ///
    /// # Examples
    ///
    /// Get url
    ///
    /// ```
    /// use kensaku::models::Document;
    ///
    /// let d = Document::new(
    ///     1,
    ///     "http://acme.com/about".to_string(),
    ///     "About Us".to_string(),
    ///     "We are Acme and we make things.".to_string(),
    /// );
    /// let url: String = d.url();
    /// ```
    pub fn url(&self) -> String {
        return self.url.to_string();
    }

    /// title() is a getter for the private field title
    ///
    /// # Examples
    ///
    /// Get title
    ///
    /// ```
    /// use kensaku::models::Document;
    ///
    /// let d = Document::new(
    ///     1,
    ///     "http://acme.com/about".to_string(),
    ///     "About Us".to_string(),
    ///     "We are Acme and we make things.".to_string(),
    /// );
    /// let title: String = d.title();
    /// ```
    pub fn title(&self) -> String {
        return self.title.to_string();
    }

    /// content() is a getter for the private field content
    ///
    /// # Examples
    ///
    /// Get content
    ///
    /// ```
    /// use kensaku::models::Document;
    ///
    /// let d = Document::new(
    ///     1,
    ///     "http://acme.com/about".to_string(),
    ///     "About Us".to_string(),
    ///     "We are Acme and we make things.".to_string(),
    /// );
    /// let content: String = d.content();
    /// ```
    pub fn content(&self) -> String {
        return self.content.to_string();
    }
}

/// Index contains information about a particular word in a document. A document
/// can only contain unique indexes. The locations field should hold all
/// occurences of the word in the document and their location in the document.
pub struct Index<'a> {
    id: i32,
    word: String,
    locations: &'a mut Vec<i32>,
}

impl<'a> Index<'a> {
    /// new() creates a new instantiation of an Index.
    ///
    /// # Examples
    ///
    /// Create Index
    ///
    /// ```
    /// use kensaku::models::Index;
    ///
    /// let mut locations = vec![1,2,3];
    /// let i = Index::new(
    ///     1,
    ///     "acme".to_string(),
    ///     &mut locations,
    /// );
    /// ```
    pub fn new(id: i32, word: String, locations: &mut Vec<i32>) -> Index {
        Index {
            id: id,
            word: word,
            locations: locations,
        }
    }

    /// id() is a getter for the private field id
    ///
    /// # Examples
    ///
    /// Get id
    ///
    /// ```
    /// use kensaku::models::Index;
    ///
    /// let mut locations = vec![1,2,3];
    /// let i = Index::new(
    ///     1,
    ///     "acme".to_string(),
    ///     &mut locations,
    /// );
    /// let id: i32 = i.id();
    /// ```
    pub fn id(&self) -> i32 {
        return self.id;
    }

    /// word() is a getter for the private field word
    ///
    /// # Examples
    ///
    /// Get word
    ///
    /// ```
    /// use kensaku::models::Index;
    ///
    /// let mut locations = vec![1,2,3];
    /// let i = Index::new(
    ///     1,
    ///     "acme".to_string(),
    ///     &mut locations,
    /// );
    /// let word: String = i.word();
    /// ```
    pub fn word(&self) -> String {
        return self.word.to_string();
    }

    /// locations() is a getter for the private field locations
    ///
    /// # Examples
    ///
    /// Get locations
    ///
    /// ```
    /// use kensaku::models::Index;
    ///
    /// let mut locations = vec![1,2,3];
    /// let i = Index::new(
    ///     1,
    ///     "acme".to_string(),
    ///     &mut locations,
    /// );
    /// let mut index_locations = i.locations();
    /// ```
    pub fn locations(&self) ->  &Vec<i32> {
        return &self.locations;
    }

    /// set_locations() is a setter for the private field locations
    ///
    /// # Examples
    ///
    /// Set locations
    ///
    /// ```
    /// use kensaku::models::Index;
    ///
    /// let mut locations = vec![1,2,3];
    /// let mut i = Index::new(
    ///     1,
    ///     "acme".to_string(),
    ///     &mut locations,
    /// );
    /// i.set_locations(4); // i.locations now equals [1,2,3,4]
    /// ```
    pub fn set_locations(&mut self, val: i32) {
        self.locations.push(val);
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
        let mut locations = vec![1,2,3];
        let i = Index::new(
            1,
            "acme".to_string(),
            &mut locations,
        );

        assert_eq!(1, i.id());
        assert_eq!("acme", i.word());
        assert_eq!(3, i.locations().len());
    }

    #[test]
    fn test_set_locations() {
        let mut locations = vec![1,2,3];
        let mut i = Index::new(
            1,
            "acme".to_string(),
            &mut locations,
        );

        assert_eq!(3, i.locations().len());

        i.set_locations(4);

        assert_eq!(4, i.locations().len());
    }
}