use crate::{
    html::HTMLElement,
    styling::{
        Color, Display, Font, FontStyle, FontWeight, Margin, Style, TextDecoration,
        TextDecorationLine, TextDecorationStyle, Unit,
    },
};

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

struct MaybeStyle {
    pub display: Option<Display>,
    pub margin: Option<Margin>,
    pub font: Option<Font>,
    pub color: Option<Color>,
    pub text_decoration: Option<TextDecoration>,
}

struct InheritableStyle {
    pub font: Font,
    pub color: Color,
    pub text_decoration: TextDecoration,
}

impl Default for InheritableStyle {
    fn default() -> Self {
        Self {
            font: Font::default(),
            color: Color::default(),
            text_decoration: TextDecoration::default(),
        }
    }
}

impl MaybeStyle {
    pub(crate) fn from_tag(tag: &str) -> Self {
        match tag {
            "p" => Self {
                display: Some(Display::Block),
                margin: Some(Margin::new(
                    Unit::Em(1.0),
                    Unit::Em(0.0),
                    Unit::Em(1.0),
                    Unit::Em(0.0),
                )),
                font: None,
                color: None,
                text_decoration: None,
            },
            "h1" => Self {
                display: Some(Display::Block),
                margin: Some(Margin::new(
                    Unit::Em(0.67),
                    Unit::Em(0.0),
                    Unit::Em(0.67),
                    Unit::Em(0.0),
                )),
                font: Some(Font::new(
                    Unit::Em(2.0),
                    "Times New Roman".to_string(),
                    FontWeight::Bold,
                    FontStyle::Normal,
                )),
                color: None,
                text_decoration: None,
            },
            "a" => Self {
                display: None,
                margin: None,
                font: None,
                color: Some(Color::new(0, 0, 238)),
                text_decoration: Some(TextDecoration {
                    color: Color::new(0, 0, 238),
                    line: TextDecorationLine::Underline,
                    style: TextDecorationStyle::Solid,
                }),
            },
            _ => Self {
                display: None,
                margin: None,
                font: None,
                color: None,
                text_decoration: None,
            },
        }
    }
}

impl HTMLElement {
    pub(crate) fn into_dom_element(self, inherited_style: &InheritableStyle) -> DOMElement {
        match self {
            HTMLElement::Element {
                tag,
                attributes,
                children,
            } => {
                // Get style
                let new_style = MaybeStyle::from_tag(&tag);
                // Inherit if not present
                let style = Style {
                    display: new_style.display.unwrap_or(Display::default()),
                    margin: new_style.margin.unwrap_or(Margin::default()),
                    font: new_style.font.unwrap_or(inherited_style.font.clone()),
                    color: new_style.color.unwrap_or(inherited_style.color),
                    text_decoration: new_style
                        .text_decoration
                        .unwrap_or(inherited_style.text_decoration.clone()),
                };
                // Create new inherited style
                let inherited_style = InheritableStyle {
                    font: style.font.clone(),
                    color: style.color.clone(),
                    text_decoration: style.text_decoration.clone(),
                };
                // Get actions
                let actions = DOMAction::from_html_element(&tag, &attributes);
                // Recurse on children
                let children = children
                    .into_iter()
                    .map(|child| child.into_dom_element(&inherited_style))
                    .collect();
                // Return DOMElement
                DOMElement::View {
                    style,
                    children,
                    actions,
                }
            }
            HTMLElement::Text(text) => {
                let style = Style {
                    display: Display::Inline,
                    margin: Margin::default(),
                    font: inherited_style.font.clone(),
                    color: inherited_style.color,
                    text_decoration: inherited_style.text_decoration.clone(),
                };

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

#[derive(Debug)]
pub(crate) struct DOM {
    pub elements: Vec<DOMElement>,
}

impl DOM {
    pub(crate) fn construct_dom(html_elements: Vec<HTMLElement>) -> Self {
        let elements = html_elements
            .into_iter()
            .map(|element| element.into_dom_element(&InheritableStyle::default()))
            .collect();
        Self { elements }
    }
}
