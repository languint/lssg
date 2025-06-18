use crate::parser::{self, MarkdownNodes, MarkdownParagraph, MarkdownSpan, MarkdownVariant};

pub struct Translator {
    nodes: Vec<MarkdownNodes>,
}

impl Translator {
    pub fn new(nodes: Vec<MarkdownNodes>) -> Self {
        Translator { nodes }
    }

    fn format_span(span: &MarkdownSpan) -> String {
        match span.variant {
            MarkdownVariant::Bold => format!("<strong>{}</strong>", span.content),
            MarkdownVariant::Italic => format!("<em>{}</em>", span.content),
            MarkdownVariant::InlineCode => format!("<code>{}</code>", span.content),
            MarkdownVariant::Normal => span.content.clone(),
        }
    }

    fn format_paragraph(paragraph: &MarkdownParagraph) -> String {
        let mut html = String::new();
        for span in &paragraph.spans {
            html.push_str(&Self::format_span(span));
        }
        html
    }

    pub fn translate(&self) -> String {
        let mut output = String::new();
        for node in &self.nodes {
            match node {
                MarkdownNodes::Paragraph(p) => {
                    output.push_str(&format!("<p>{}</p>", Self::format_paragraph(p)));
                }
                MarkdownNodes::Heading(h) => {
                    output.push_str(&format!("<h{0}>{1}</h{0}>", h.level, h.content));
                }
                MarkdownNodes::Link(l) => {
                    let node = match l.is_image {
                        true => format!("<img src=\"{}\" alt=\"{}\" />", l.url, l.alt),
                        false => format!("<a href=\"{}\">{}</a>", l.url, l.alt),
                    };
                    output.push_str(&node);
                }
                MarkdownNodes::List(l) => {
                    let node = match l.ty {
                        parser::MarkdownListType::Numbered => "ol",
                        parser::MarkdownListType::Bulleted => "ul",
                    };

                    let items = l
                        .items
                        .iter()
                        .map(|i| format!("<li>{}</li>", i))
                        .collect::<String>();

                    let out = format!("<{}>{}</{}>", node, items.as_str(), node);

                    output.push_str(&out);
                }
                _ => {}
            }
        }
        output
    }
}
