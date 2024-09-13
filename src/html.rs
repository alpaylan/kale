


#[derive(Debug)]
pub(crate) struct HTMLElement {
    pub tag: String,
    pub id: String,
    pub inner_text: String,
    pub children: Vec<HTMLElement>,
}

impl HTMLElement {
    pub(crate) fn new(tag: String, id: String, inner_text: String, children: Vec<HTMLElement>) -> Self {
        Self {
            tag,
            id,
            inner_text,
            children,
        }
    }
}

impl ToString for HTMLElement {
    fn to_string(&self) -> String {
        let mut s = format!("<{} id=\"{}\">{}</{}>", self.tag, self.id, self.inner_text, self.tag);
        for child in &self.children {
            s.push_str(&child.to_string());
        }
        s
    }
}




