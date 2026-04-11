use crate::core::document::Document;
use crate::parser::semantic_tree::SemanticTree;

pub struct Serializer;

impl Serializer {
    pub fn to_markdown(doc: &Document) -> String {
        SemanticTree::to_markdown(doc)
    }

    pub fn to_html(doc: &Document) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; ");
        html.push_str("max-width: 800px; margin: 0 auto; padding: 2rem; line-height: 1.6; }\n");
        html.push_str("h1, h2, h3, h4, h5, h6 { margin-top: 1.5em; margin-bottom: 0.5em; }\n");
        html.push_str("code { background: #f4f4f4; padding: 0.2em 0.4em; border-radius: 3px; }\n");
        html.push_str("pre { background: #f4f4f4; padding: 1em; overflow-x: auto; }\n");
        html.push_str("blockquote { border-left: 4px solid #ddd; margin: 0; padding-left: 1em; color: #666; }\n");
        html.push_str("a { color: #0066cc; }\n");
        html.push_str("img { max-width: 100%; }\n");
        html.push_str("table { border-collapse: collapse; width: 100%; }\n");
        html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        html.push_str("</style>\n</head>\n<body>\n");
        html.push_str(&Self::node_to_html(&doc.root));
        html.push_str("\n</body>\n</html>");
        html
    }

    fn node_to_html(node: &crate::core::document::Node) -> String {
        use crate::core::document::NodeKind;
        let mut html = String::new();
        match &node.kind {
            NodeKind::Document => {
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
            }
            NodeKind::Heading { level } => {
                html.push_str(&format!("<h{}>", level));
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str(&format!("</h{}>\n", level));
            }
            NodeKind::Paragraph => {
                html.push_str("<p>");
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</p>\n");
            }
            NodeKind::Bold => {
                html.push_str("<strong>");
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</strong>");
            }
            NodeKind::Italic => {
                html.push_str("<em>");
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</em>");
            }
            NodeKind::Strikethrough => {
                html.push_str("<del>");
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</del>");
            }
            NodeKind::InlineCode => {
                html.push_str("<code>");
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</code>");
            }
            NodeKind::CodeBlock { lang: _ } => {
                html.push_str("<pre><code>");
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</code></pre>\n");
            }
            NodeKind::BlockQuote => {
                html.push_str("<blockquote>");
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</blockquote>\n");
            }
            NodeKind::List { .. } => {
                html.push_str("<ul>\n");
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</ul>\n");
            }
            NodeKind::ListItem { .. } => {
                html.push_str("<li>");
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</li>\n");
            }
            NodeKind::Link { dest } => {
                html.push_str(&format!("<a href=\"{}\">", dest));
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
                html.push_str("</a>");
            }
            NodeKind::Image { alt, dest } => {
                html.push_str(&format!("<img src=\"{}\" alt=\"{}\">", dest, alt));
            }
            NodeKind::HorizontalRule => {
                html.push_str("<hr>\n");
            }
            NodeKind::SoftBreak | NodeKind::HardBreak => {
                html.push_str("<br>\n");
            }
            _ => {
                for child in &node.children {
                    html.push_str(&Self::node_to_html(child));
                }
            }
        }
        html
    }
}
