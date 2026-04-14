# PDF Code Block Test Fixture

## TC-PDF002: Multi-language Code Blocks with Syntax Highlighting

### Rust Code
```rust
use std::collections::HashMap;

struct MarkdownParser {
    tokens: Vec<Token>,
    position: usize,
}

impl MarkdownParser {
    fn new() -> Self {
        Self {
            tokens: Vec::new(),
            position: 0,
        }
    }

    fn parse(&mut self, input: &str) -> Result<Document, ParseError> {
        let mut lines = input.lines();
        while let Some(line) = lines.next() {
            if line.starts_with('#') {
                self.parse_heading(line)?;
            } else if line.starts_with("```") {
                self.parse_code_block(&mut lines)?;
            }
        }
        Ok(Document::new(self.tokens.clone()))
    }
}
```

### JavaScript Code
```javascript
class MarkdownRenderer {
    constructor(options = {}) {
        this.options = {
            highlight: true,
            breaks: false,
            ...options
        };
        this.headings = [];
    }

    parse(markdown) {
        const lines = markdown.split('\n');
        const tokens = [];
        
        for (const line of lines) {
            if (line.startsWith('#')) {
                tokens.push(this.parseHeading(line));
            } else if (line.startsWith('```')) {
                tokens.push(this.parseCodeBlock(lines));
            } else {
                tokens.push({ type: 'paragraph', content: line });
            }
        }
        
        return { tokens, headings: this.headings };
    }
}
```

### Python Code
```python
from dataclasses import dataclass
from typing import List, Optional
import re

@dataclass
class Token:
    type: str
    content: str
    meta: Optional[dict] = None

class MarkdownParser:
    def __init__(self):
        self.tokens: List[Token] = []
        self.headings: List[dict] = []
    
    def parse(self, text: str) -> 'Document':
        lines = text.split('\n')
        
        in_code_block = False
        code_buffer = []
        code_language = None
        
        for line in lines:
            if line.startswith('```'):
                if not in_code_block:
                    in_code_block = True
                    code_language = line[3:].strip() or 'text'
                    code_buffer = []
                else:
                    in_code_block = False
                    self.tokens.append(Token(
                        type='code',
                        content='\n'.join(code_buffer),
                        meta={'language': code_language}
                    ))
                continue
            
            if in_code_block:
                code_buffer.append(line)
            elif line.startswith('#'):
                self.parse_heading(line)
            else:
                self.tokens.append(Token(type='paragraph', content=line))
        
        return Document(tokens=self.tokens)
```

### Go Code
```go
package main

import (
    "fmt"
    "strings"
)

type Token struct {
    Type    string
    Content string
    Meta    map[string]interface{}
}

type Parser struct {
    tokens   []Token
    position int
}

func NewParser() *Parser {
    return &Parser{
        tokens:   make([]Token, 0),
        position: 0,
    }
}

func (p *Parser) Parse(input string) error {
    lines := strings.Split(input, "\n")
    
    for _, line := range lines {
        if strings.HasPrefix(line, "#") {
            if err := p.parseHeading(line); err != nil {
                return err
            }
        } else if strings.HasPrefix(line, "```") {
            if err := p.parseCodeBlock(lines); err != nil {
                return err
            }
        } else {
            p.tokens = append(p.tokens, Token{
                Type:    "paragraph",
                Content: line,
            })
        }
    }
    
    return nil
}
```

### Mixed Inline Code
This paragraph contains `inline code` and more `code fragments` scattered throughout.

### Code with Special Characters
```javascript
const template = `Template literal with ${variable}`;
const escaped = "String with \"quotes\" and \\ backslash";
const html = '<div class="container">Content</div>';
```

### Empty Code Block
```rust
```

### Code Block with Unicode
```python
def greet():
    print("你好，世界！🌍")
    print("Привет мир！")
    print("مرحبا بالعالم！")
```
