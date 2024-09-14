use crate::{html::HTMLElement, styling::Style};

#[derive(Debug, Clone)]
pub(crate) enum DOMElement {
    View {
        style: Style,
        children: Vec<DOMElement>,
        actions: Vec<DOMAction>,
    },
    Text {
        style: Style,
        text: String,
        actions: Vec<DOMAction>,
    },
}

impl DOMElement {
    pub(crate) fn style(&self) -> &Style {
        match self {
            Self::View { style, .. } => style,
            Self::Text { style, .. } => style,
        }
    }
    pub(crate) fn actions(&self) -> &Vec<DOMAction> {
        match self {
            Self::View { actions, .. } => actions,
            Self::Text { actions, .. } => actions,
        }
    }
}

impl DOMElement {
    pub(crate) fn unchecked_text(&self) -> String {
        match self {
            DOMElement::View {
                style,
                children,
                actions,
            } => {
                panic!("Never call with a view")
            }
            DOMElement::Text { text, .. } => text.clone(),
        }
    }

    pub(crate) fn to_text(&self) -> Self {
        match self {
            DOMElement::View {
                style,
                children,
                actions,
            } => {
                assert!(children.len() == 1);

                DOMElement::Text {
                    style: style.clone(),
                    text: children[0].unchecked_text(),
                    actions: actions.clone(),
                }
            }
            DOMElement::Text { .. } => self.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum DOMAction {
    ClickToRedirect(String),
}

impl DOMAction {
    pub(crate) fn from_html_element(tag: &str, attributes: &Vec<(String, String)>) -> Vec<Self> {
        match tag {
            "a" => {
                let mut actions = vec![];
                for (key, value) in attributes {
                    if key == "href" {
                        actions.push(DOMAction::ClickToRedirect(value.clone()));
                    }
                }
                actions
            }
            _ => vec![],
        }
    }
}

impl From<HTMLElement> for DOMElement {
    fn from(value: HTMLElement) -> Self {
        match value {
            HTMLElement::Element {
                tag,
                attributes,
                children,
            } => {
                // Get style
                let style = Style::from_tag(&tag);
                // Get actions
                let actions = DOMAction::from_html_element(&tag, &attributes);
                // Recurse on children
                let children = children.into_iter().map(|child| child.into()).collect();
                // Return DOMElement
                DOMElement::View {
                    style,
                    children,
                    actions,
                }
            }
            HTMLElement::Text(text) => {
                let style = Style::default();

                let actions = vec![];
                DOMElement::Text {
                    style,
                    text,
                    actions,
                }
            }
        }
    }
}

pub(crate) struct DOM {
    pub elements: Vec<DOMElement>,
}

impl DOM {
    pub(crate) fn construct_dom(html_elements: Vec<HTMLElement>) -> Self {
        let elements = html_elements
            .into_iter()
            .map(|element| element.into())
            .collect();
        Self { elements }
    }
}
