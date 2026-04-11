use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Document,
    Heading { level: u8 },
    Paragraph,
    Bold,
    Italic,
    Strikethrough,
    InlineCode,
    CodeBlock { lang: Option<String> },
    BlockQuote,
    List { kind: ListKind, tight: bool },
    ListItem { kind: ListKind },
    TaskItem { checked: bool },
    Table { cols: usize, header_row: bool },
    TableRow,
    TableCell,
    Link { dest: String },
    Image { alt: String, dest: String },
    HorizontalRule,
    SoftBreak,
    HardBreak,
    Frontmatter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Unordered,
    Ordered,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub range: Range<usize>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(kind: NodeKind, range: Range<usize>) -> Self {
        Self {
            kind,
            range,
            children: Vec::new(),
        }
    }

    pub fn with_children(mut self, children: Vec<Node>) -> Self {
        self.children = children;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub root: Node,
    pub source_len: usize,
}

impl Document {
    pub fn new(root: Node, source_len: usize) -> Self {
        Self { root, source_len }
    }

    pub fn empty() -> Self {
        Self {
            root: Node::new(NodeKind::Document, 0..0),
            source_len: 0,
        }
    }
}
