pub struct Document {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub content: String,
}

impl Document {
    pub fn new(id: i32, url: String, title: String, content: String) -> Document {
        Document {
            id: id,
            url: url,
            title: title,
            content: content,
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
}