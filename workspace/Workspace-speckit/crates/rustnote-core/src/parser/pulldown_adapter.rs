use crate::core::document::{Document, ListKind, Node, NodeKind};
use pulldown_cmark::{Event, Tag, TagEnd};

pub struct PulldownAdapter;

impl PulldownAdapter {
    pub fn parse(content: &str) -> Document {
        let parser = pulldown_cmark::Parser::new_ext(content, pulldown_cmark::Options::all());
        let root = Self::parse_events(parser, content);
        Document::new(root, content.len())
    }

    fn parse_events<'a, I: Iterator<Item = Event<'a>>>(mut iter: I, content: &'a str) -> Node {
        let mut root = Node::new(NodeKind::Document, 0..content.len());
        let mut stack: Vec<Node> = vec![root];
        let mut pos = 0;

        while let Some(event) = iter.next() {
            match event {
                Event::Start(tag) => {
                    let node = Self::create_node_from_tag(&tag, pos);
                    pos = Self::get_tag_end_pos(&tag, &mut iter);
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node.clone());
                    }
                    stack.push(node);
                }
                Event::End(TagEnd::End(tag)) => {
                    if let Some(node) = stack.pop() {
                        if let Some(parent) = stack.last_mut() {
                            parent.children.push(node);
                        }
                    }
                }
                _ => {}
            }
        }

        stack.remove(0)
    }

    fn create_node_from_tag(tag: &Tag, _pos: usize) -> Node {
        match tag {
            Tag::Document => Node::new(NodeKind::Document, 0..0),
            Tag::Heading { level } => Node::new(NodeKind::Heading { level: *level }, 0..0),
            Tag::Paragraph => Node::new(NodeKind::Paragraph, 0..0),
            Tag::Emphasis => Node::new(NodeKind::Italic, 0..0),
            Tag::Strong => Node::new(NodeKind::Bold, 0..0),
            Tag::Strikethrough => Node::new(NodeKind::Strikethrough, 0..0),
            Tag::CodeBlock(kind) => {
                let lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => Some(lang.to_string()),
                    pulldown_cmark::CodeBlockKind::Indented => None,
                };
                Node::new(NodeKind::CodeBlock { lang }, 0..0)
            }
            Tag::BlockQuote => Node::new(NodeKind::BlockQuote, 0..0),
            Tag::List(ordered) => {
                let kind = if *ordered {
                    ListKind::Ordered
                } else {
                    ListKind::Unordered
                };
                Node::new(NodeKind::List { kind, tight: false }, 0..0)
            }
            Tag::Item => Node::new(
                NodeKind::ListItem {
                    kind: ListKind::Unordered,
                },
                0..0,
            ),
            Tag::Table(_) => Node::new(
                NodeKind::Table {
                    cols: 0,
                    header_row: true,
                },
                0..0,
            ),
            Tag::TableHead => Node::new(NodeKind::TableRow, 0..0),
            Tag::TableRow => Node::new(NodeKind::TableRow, 0..0),
            Tag::TableCell => Node::new(NodeKind::TableCell, 0..0),
            Tag::Link { dest, .. } => Node::new(
                NodeKind::Link {
                    dest: dest.to_string(),
                },
                0..0,
            ),
            Tag::Image { dest, .. } => Node::new(
                NodeKind::Image {
                    alt: String::new(),
                    dest: dest.to_string(),
                },
                0..0,
            ),
            Tag::HtmlBlock => Node::new(NodeKind::Frontmatter, 0..0),
            Tag::Paragraph => Node::new(NodeKind::Paragraph, 0..0),
            Tag::Heading { .. } => Node::new(NodeKind::Paragraph, 0..0),
            _ => Node::new(NodeKind::Paragraph, 0..0),
        }
    }

    fn get_tag_end_pos<'a, I: Iterator<Item = Event<'a>>>(_tag: &Tag, _iter: &mut I) -> usize {
        0
    }
}

pub fn parse_markdown(content: &str) -> Document {
    PulldownAdapter::parse(content)
}
