use crate::{
    html::HTMLElement,
    styling::{Color, Display, Font, FontStyle, FontWeight, Margin, Style, TextDecoration, TextDecorationLine, TextDecorationStyle, Unit},
};

pub(crate) struct Paragraph {
    pub spans: Vec<Span>,
}

pub(crate) enum Span {
    Text(String),
    Anchor(Anchor),
}

pub(crate) struct Heading {
    pub level: u8,
    pub text: String,
}

pub(crate) struct Anchor {
    pub href: String,
    pub text: String,
    pub clicked: bool,
}

pub(crate) struct DescriptionList {
    content: Vec<(DescriptionTerm, DescriptionDetails)>,
}

type DescriptionDetails = DOMElement;
type DescriptionTerm = DOMElement;

pub(crate) enum DOMElement {
    Paragraph(Paragraph),
    Heading(Heading),
    Anchor(Anchor),
    DescriptionList(DescriptionList),
}

impl From<HTMLElement> for DOMElement {
    fn from(value: HTMLElement) -> Self {
        match value {
            HTMLElement::Element {
                tag,
                attributes,
                children,
            } => match tag.as_str() {
                "p" => {
                    let mut spans = Vec::new();
                    for child in children {
                        match child {
                            HTMLElement::Text(text) => {
                                spans.push(Span::Text(text));
                            }
                            HTMLElement::Element {
                                tag,
                                attributes,
                                children,
                            } => match tag.as_str() {
                                "a" => {
                                    let href = attributes
                                        .iter()
                                        .find(|(name, _)| name == "href")
                                        .unwrap()
                                        .1
                                        .clone();
                                    let text = children
                                        .iter()
                                        .find_map(|child| {
                                            if let HTMLElement::Text(text) = child {
                                                Some(text.clone())
                                            } else {
                                                None
                                            }
                                        })
                                        .unwrap();
                                    spans.push(Span::Anchor(Anchor { href, text, clicked: false }));
                                }
                                _ => todo!(),
                            },
                        }
                    }

                    DOMElement::Paragraph(Paragraph { spans })
                }
                "h1" => {
                    let text = children
                        .iter()
                        .find_map(|child| {
                            if let HTMLElement::Text(text) = child {
                                Some(text.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    DOMElement::Heading(Heading { level: 1, text })
                }
                "a" => {
                    let href = attributes
                        .iter()
                        .find(|(name, _)| name == "href")
                        .unwrap()
                        .1
                        .clone();
                    let text = children
                        .iter()
                        .find_map(|child| {
                            if let HTMLElement::Text(text) = child {
                                Some(text.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    DOMElement::Anchor(Anchor { href, text, clicked: false })
                }
                _ => todo!(),
            },
            HTMLElement::Text(_) => todo!(),
        }
    }
}

impl DOMElement {
    pub(crate) fn style(&self) -> Style {
        match self {
            DOMElement::Paragraph(_) => Style {
                display: Display::Block,
                margin: Margin::new(Unit::Em(1.0), Unit::Em(0.0), Unit::Em(1.0), Unit::Em(0.0)),
                font: Font::default(),
                color: Color::default(),
                text_decoration: TextDecoration::default(),
            },
            DOMElement::Heading(heading) => match heading.level {
                1 => Style {
                    display: Display::Block,
                    margin: Margin::new(
                        Unit::Em(0.67),
                        Unit::Em(0.0),
                        Unit::Em(0.67),
                        Unit::Em(0.0),
                    ),
                    font: Font {
                        size: Unit::Em(2.0),
                        weight: FontWeight::Bold,
                        style: FontStyle::Normal,
                        family: "Times New Roman".to_string(),
                    },
                    color: Color::default(),
                    text_decoration: TextDecoration::default(),
                },
                _ => todo!(),
            },
            DOMElement::Anchor(a) => {
                Style {
                    display: Display::Inline,
                    margin: Margin::new(Unit::Em(0.0), Unit::Em(0.0), Unit::Em(0.0), Unit::Em(0.0)),
                    font: Font::default(),
                    color: Color::default(),
                    text_decoration: TextDecoration {
                        color: if a.clicked {
                            Color::new(0, 0, 238) // Purple
                        } else {
                            Color::new(0, 0, 255) // Blue
                        },
                        line: TextDecorationLine::Underline,
                        style: TextDecorationStyle::Solid,
                    },
                }
            }
            DOMElement::DescriptionList(_) => todo!(),
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
