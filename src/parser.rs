use logos::Logos;

#[derive(Debug)]
pub struct MarkdownCodeBlock {
    pub language: String,
    pub content: String,
}

#[derive(Debug)]
pub struct MarkdownHeading {
    pub level: u8,
    pub content: String,
}

#[derive(Debug)]
pub enum MarkdownVariant {
    Bold,
    Italic,
    Normal,
    InlineCode,
}

#[derive(Debug)]
pub enum MarkdownListType {
    Numbered,
    Bulleted,
}

#[derive(Debug)]
pub struct MarkdownList {
    pub ty: MarkdownListType,
    pub items: Vec<String>,
}

#[derive(Debug)]
pub enum MarkdownNodes {
    Heading(MarkdownHeading),
    Paragraph(MarkdownParagraph),
    List(MarkdownList),
    Link(MarkdownLink),
    CodeBlock(MarkdownCodeBlock),
    HorizontalRule,
}

#[derive(Logos)]
#[logos(extras = (usize, usize))]
pub enum Token {
    #[token("#")]
    Hash,
    #[token("*")]
    Asterisk,
    #[token("-")]
    Dash,
    #[token(" ")]
    Space,
    #[token("!")]
    ExclamationMark,
    #[token("`")]
    Backtick,
    #[regex(r"[a-zA-Z0-9]+")]
    Word,
}

#[derive(Debug)]
pub struct MarkdownSpan {
    pub content: String,
    pub variant: MarkdownVariant,
}

#[derive(Debug)]
pub struct MarkdownParagraph {
    pub spans: Vec<MarkdownSpan>,
}

#[derive(Debug)]
pub struct MarkdownLink {
    pub alt: String,
    pub url: String,
    pub is_image: bool,
}

pub struct MarkdownParser {
    input: String,
}

impl MarkdownParser {
    pub fn new(input: String) -> Self {
        MarkdownParser { input }
    }

    fn parse_inline_formatting(&self, text: &str) -> Vec<MarkdownSpan> {
        let mut spans = Vec::new();
        let mut chars = text.chars().peekable();
        let mut buffer = String::new();

        enum State {
            Normal,
            Italic,
            Bold,
            InlineCode,
        }

        let mut state = State::Normal;

        while let Some(c) = chars.next() {
            match c {
                '*' => {
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        if !buffer.is_empty() {
                            spans.push(MarkdownSpan {
                                content: buffer.clone(),
                                variant: match state {
                                    State::Normal => MarkdownVariant::Normal,
                                    State::Italic => MarkdownVariant::Italic,
                                    State::Bold => MarkdownVariant::Bold,
                                    State::InlineCode => MarkdownVariant::InlineCode,
                                },
                            });
                            buffer.clear();
                        }
                        state = match state {
                            State::Bold => State::Normal,
                            _ => State::Bold,
                        };
                    } else {
                        if !buffer.is_empty() {
                            spans.push(MarkdownSpan {
                                content: buffer.clone(),
                                variant: match state {
                                    State::Normal => MarkdownVariant::Normal,
                                    State::Italic => MarkdownVariant::Italic,
                                    State::Bold => MarkdownVariant::Bold,
                                    State::InlineCode => MarkdownVariant::InlineCode,
                                },
                            });
                            buffer.clear();
                        }
                        state = match state {
                            State::Italic => State::Normal,
                            _ => State::Italic,
                        };
                    }
                }
                '`' => {
                    if !buffer.is_empty() {
                        spans.push(MarkdownSpan {
                            content: buffer.clone(),
                            variant: match state {
                                State::Normal => MarkdownVariant::Normal,
                                State::Italic => MarkdownVariant::Italic,
                                State::Bold => MarkdownVariant::Bold,
                                State::InlineCode => MarkdownVariant::InlineCode,
                            },
                        });
                        buffer.clear();
                    }
                    state = match state {
                        State::InlineCode => State::Normal,
                        _ => State::InlineCode,
                    };
                }
                _ => buffer.push(c),
            }
        }

        if !buffer.is_empty() {
            spans.push(MarkdownSpan {
                content: buffer,
                variant: match state {
                    State::Normal => MarkdownVariant::Normal,
                    State::Italic => MarkdownVariant::Italic,
                    State::Bold => MarkdownVariant::Bold,
                    State::InlineCode => MarkdownVariant::InlineCode,
                },
            });
        }

        spans
    }

    pub fn parse(&self) -> Vec<MarkdownNodes> {
        let mut nodes = Vec::new();
        let mut lines = self.input.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim_start();

            if trimmed.starts_with("---") {
                nodes.push(MarkdownNodes::HorizontalRule)
            } else if trimmed.starts_with("###") {
                nodes.push(MarkdownNodes::Heading(MarkdownHeading {
                    level: 3,
                    content: trimmed[3..].trim().to_string(),
                }));
            } else if trimmed.starts_with("##") {
                nodes.push(MarkdownNodes::Heading(MarkdownHeading {
                    level: 2,
                    content: trimmed[2..].trim().to_string(),
                }));
            } else if trimmed.starts_with("#") {
                nodes.push(MarkdownNodes::Heading(MarkdownHeading {
                    level: 1,
                    content: trimmed[1..].trim().to_string(),
                }));
            } else if trimmed.starts_with("```") {
                let lang = trimmed[3..].trim().to_string();
                let mut content = String::new();
                while let Some(next) = lines.next() {
                    if next.trim() == "```" {
                        break;
                    } else {
                        content.push_str(next);
                        content.push('\n');
                    }
                }
                nodes.push(MarkdownNodes::CodeBlock(MarkdownCodeBlock {
                    language: lang,
                    content,
                }));
            } else if trimmed.starts_with("-") {
                let mut items = vec![trimmed[1..].trim().to_string()];
                while let Some(peeked) = lines.peek() {
                    if peeked.trim_start().starts_with("-") {
                        let item_line = lines.next().unwrap();
                        items.push(item_line.trim_start()[1..].trim().to_string());
                    } else {
                        break;
                    }
                }
                nodes.push(MarkdownNodes::List(MarkdownList {
                    ty: MarkdownListType::Bulleted,
                    items,
                }));
            } else if trimmed
                .chars()
                .next()
                .map(|c| c.is_digit(10))
                .unwrap_or(false)
                && trimmed.contains('.')
            {
                let mut items = vec![trimmed.split_once('.').unwrap().1.trim().to_string()];
                while let Some(peeked) = lines.peek() {
                    let trimmed_peek = peeked.trim_start();
                    if let Some((_, rest)) = trimmed_peek.split_once('.') {
                        if trimmed_peek.chars().next().unwrap().is_digit(10) {
                            lines.next();
                            items.push(rest.trim().to_string());
                            continue;
                        }
                    }
                    break;
                }
                nodes.push(MarkdownNodes::List(MarkdownList {
                    ty: MarkdownListType::Numbered,
                    items,
                }));
            } else if trimmed.starts_with("![") || trimmed.starts_with("[") {
                // Parse link with proper markdown syntax
                let is_image = trimmed.starts_with("![");
                let start_pos = if is_image { 2 } else { 1 };

                if let Some(text_end) = trimmed[start_pos..].find(']') {
                    let alt_text = &trimmed[start_pos..start_pos + text_end];
                    let remaining = &trimmed[start_pos + text_end + 1..];

                    if remaining.starts_with('(') {
                        if let Some(url_end) = remaining.find(')') {
                            let url = &remaining[1..url_end];
                            nodes.push(MarkdownNodes::Link(MarkdownLink {
                                alt: alt_text.to_string(),
                                url: url.to_string(),
                                is_image,
                            }));
                        }
                    }
                }
            } else if !trimmed.is_empty() {
                let spans = self.parse_inline_formatting(trimmed);
                nodes.push(MarkdownNodes::Paragraph(MarkdownParagraph { spans }));
            }
        }

        nodes
    }
}
