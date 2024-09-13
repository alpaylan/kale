use crate::html::HTMLElement;

enum ParserState {
    SeekingElementStart,
    ReadingElementTag,
    ReadingElementProperties,
}

pub(crate) fn parse(html: &str) -> Vec<HTMLElement> {
    let mut elements = vec![];

    let mut element: Option<HTMLElement> = None;
    let mut parser_state = ParserState::SeekingElementStart;

    for c in html.chars() {
        match parser_state {
            ParserState::SeekingElementStart => match c {
                '<' => {
                    parser_state = ParserState::ReadingElementTag;
                    element = Some(HTMLElement::new(
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        vec![],
                    ));
                }
                ' ' | '\t' | '\n' => {}
                _ => {
                    panic!("Unexpected character: {}", c);
                }
            },
            ParserState::ReadingElementTag => todo!(),
            ParserState::ReadingElementProperties => todo!(),
        }
    }

    elements
}
