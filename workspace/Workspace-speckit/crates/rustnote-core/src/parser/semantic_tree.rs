use crate::core::document::{Document, ListKind, Node, NodeKind};

pub struct SemanticTree;

impl SemanticTree {
    pub fn from_markdown(doc: &Document) -> Node {
        doc.root.clone()
    }

    pub fn get_headings(doc: &Document) -> Vec<HeadingInfo> {
        let mut headings = Vec::new();
        Self::collect_headings(&doc.root, &mut headings, 0);
        headings
    }

    fn collect_headings(node: &Node, headings: &mut Vec<HeadingInfo>, depth: usize) {
        if let NodeKind::Heading { level } = &node.kind {
            headings.push(HeadingInfo {
                level: *level,
                text: String::new(),
            });
        }
        for child in &node.children {
            Self::collect_headings(child, headings, depth + 1);
        }
    }

    pub fn to_markdown(doc: &Document) -> String {
        Self::node_to_markdown(&doc.root, 0)
    }

    fn node_to_markdown(node: &Node, indent: usize) -> String {
        let mut result = String::new();
        match &node.kind {
            NodeKind::Document => {
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                    if !result.ends_with('\n') {
                        result.push('\n');
                    }
                }
            }
            NodeKind::Heading { level } => {
                result.push_str(&"#".repeat(*level as usize));
                result.push(' ');
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
                result.push('\n');
            }
            NodeKind::Paragraph => {
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
                result.push('\n');
            }
            NodeKind::Bold => {
                result.push_str("**");
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
                result.push_str("**");
            }
            NodeKind::Italic => {
                result.push('*');
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
                result.push('*');
            }
            NodeKind::Strikethrough => {
                result.push_str("~~");
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
                result.push_str("~~");
            }
            NodeKind::InlineCode => {
                result.push('`');
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
                result.push('`');
            }
            NodeKind::CodeBlock { lang } => {
                result.push_str("```");
                if let Some(l) = lang {
                    result.push_str(l);
                }
                result.push('\n');
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
                result.push_str("\n```\n");
            }
            NodeKind::BlockQuote => {
                for child in &node.children {
                    result.push_str("> ");
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
            }
            NodeKind::List { kind, tight: _ } => {
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
                result.push('\n');
            }
            NodeKind::ListItem { kind } => {
                let marker = match kind {
                    ListKind::Unordered => "- ",
                    ListKind::Ordered => "1. ",
                };
                result.push_str(marker);
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent + 2));
                }
            }
            NodeKind::HorizontalRule => {
                result.push_str("---\n");
            }
            _ => {
                for child in &node.children {
                    result.push_str(&Self::node_to_markdown(child, indent));
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct HeadingInfo {
    pub level: u8,
    pub text: String,
}
