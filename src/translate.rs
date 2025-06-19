use crate::parser::{self, MarkdownNodes, MarkdownParagraph, MarkdownSpan, MarkdownVariant};

pub struct Translator<'a> {
    nodes: &'a Vec<MarkdownNodes>,
}

impl<'a> Translator<'a> {
    pub fn new(nodes: &'a Vec<MarkdownNodes>) -> Self {
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

    pub fn translate(&self, class: &str) -> String {
        let mut output = String::new();
        for node in self.nodes {
            match node {
                MarkdownNodes::Paragraph(p) => {
                    output.push_str(&format!(
                        "<p class=\"{}\">{}</p>",
                        class,
                        Self::format_paragraph(p)
                    ));
                }
                MarkdownNodes::Heading(h) => {
                    output.push_str(&format!(
                        "<h{} class=\"{}\">{}</h{}>",
                        h.level, class, h.content, h.level
                    ));
                }
                MarkdownNodes::Link(l) => {
                    let node = match l.is_image {
                        true => format!(
                            "<img src=\"{}\" alt=\"{}\" class=\"{}\" />",
                            l.url, l.alt, class
                        ),
                        false => format!("<a href=\"{}\" class=\"{}\">{}</a>", l.url, class, l.alt),
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
                        .map(|i| format!("<li class=\"{}\">{}</li>", class, i))
                        .collect::<String>();

                    let out = format!(
                        "<{} class=\"{}\">{}</{}>",
                        node,
                        class,
                        items.as_str(),
                        node
                    );

                    output.push_str(&out);
                }
                MarkdownNodes::CodeBlock(c) => {
                    let node = format!(
                        "<pre><code class=\"language-{} {}\">{}</code></pre>",
                        c.language, class, c.content
                    );
                    output.push_str(&node);
                }
                MarkdownNodes::HorizontalRule => {
                    let node = format!("<hr class=\"{}\" />", class);
                    output.push_str(&node);
                }
                _ => {}
            }
        }
        output
    }
}
