# PDF Nested Structures Test Fixture

## TC-PDF005: Nested Blockquotes, Lists, and Code

# Level 1 Heading

## Level 2 Heading

### Level 3 Heading

#### Level 4 Heading

##### Level 5 Heading

###### Level 6 Heading

---

## 5-Level Nested Blockquotes

### Blockquote Level 1
> This is a level 1 blockquote

### Blockquote Level 2
> Outer blockquote
>> Inner blockquote
>>> Deeper blockquote

### Blockquote Level 3
> **Bold in blockquote**
>> *Italic in nested*
>>> `Code in deep nested`

### Blockquote Level 4
> > > > This is deeply nested

### Blockquote Level 5
> L1
>> L2
>>> L3
>>>> L4
>>>>> L5 deepest

---

## 5-Level Nested Lists

### List Level 1
- Item 1
- Item 2

### List Level 2
- L1 Item
  - L2 Item
    - L3 Item

### List Level 3
- First level
  - Second level
    - Third level
      - Fourth level
        - Fifth level

### Ordered List Levels
1. Level 1
   1. Level 2
      1. Level 3
         1. Level 4
            1. Level 5

### Mixed Nested Lists
- Text item
  1. Numbered sub-item
     - Bullet sub-sub-item
       1. Numbered sub-sub-sub-item
          - Bullet at max depth

---

## 5-Level Nested Code Blocks

### Code in Blockquotes
> ```rust
> fn main() {
>     println!("Hello!");
> }
> ```

### Blockquote with Code and Text
> This is outer
>> Nested with code:
>> ```javascript
>> console.log("test");
>> ```
>> Back to text

### Deep Code Nesting
> Level 1
>> ```python
>> # Level 2 code
>> def nested():
>>     # Level 3
>>     pass
>> ```
>>> Level 3 text

---

## Nested Task Lists

- [x] Completed L1
  - [x] Completed L2
    - [x] Completed L3
      - [x] Completed L4
        - [x] Completed L5

- [ ] Pending L1
  - [ ] Pending L2
    - [ ] Pending L3
      - [ ] Pending L4
        - [ ] Pending L5

---

## Complex Nested Structures

### Blockquote > List > Code > Blockquote
> - Item 1
>   - Item 2
>     ```rust
>     // Code in list in blockquote
>     fn nested();
>     ```
>     > Nested blockquote in list in blockquote

### List > Blockquote > List
- L1 Item
  > B1 Text
  - L2 in list
    > B2 Nested

### Tables with Nested Content
| Outer | Contains |
|-------|----------|
| Text | - Item 1 |
| Code | `inline` |
| Quote | > Nested |

---

## Maximum Depth Examples

### Maximum List Depth
- 1
  - 2
    - 3
      - 4
        - 5
          - 6 (max practical)

### Maximum Blockquote Depth
> L1
>> L2
>>> L3
>>>> L4
>>>>> L5

### Maximum Combined Depth
> **Bold**
>> *Italic*
>>> `Code`
>>>> **Bold Again**
>>>>> *Italic Again*

---

## Real-World Example: Documentation Style

> **Note:** This is an important note.
> 
> For example:
> ```rust
> // Configuration
> let config = Config {
>     timeout: 30,
>     retries: 3,
> };
> ```
> 
> See also:
> - [Reference 1](#)
> - [Reference 2](#)
>   - [Sub-reference](#)
>
> > **Warning:** Be careful with nested quotes.
> > They can become hard to read.

---

## Final Test

This document tests 5-level nesting of:
1. Headings (H1-H6)
2. Blockquotes
3. Lists (bullet and numbered)
4. Task lists
5. Code blocks
6. Combined complex structures

The PDF should render each nesting level visually distinguishable.
