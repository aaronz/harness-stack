# Large Document (1000+ Paragraphs)

This is a generated document with exactly 1000 paragraphs for performance testing.

{% for i in range(1, 1001) %}
## Section {{ i }}

This is paragraph {{ i }}. It contains some text to simulate a real document. The purpose of this document is to test RustNote's performance with a large amount of content.

- Item {{ i }}.1
- Item {{ i }}.2
- Item {{ i }}.3

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.

**Bold text** and *italic text* and `code text` in paragraph {{ i }}.
{% endfor %}