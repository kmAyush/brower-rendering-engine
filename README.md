# HTML/CSS Rendering Engine
<p><strong>Trying to understand how browser renders HTML and CSS, and is it required to have two extra languages for rendering, can one build webpage in one single language without transpiling to HTML?</strong></p><p>
A toy implementation of browser rendering engine to understand the execution of HTML and CSS code as well as to learn about Rust data structure by practical implementation of HashMaps, Nodes and Trees.</p>

# Simplified Architecture

<div align-content="center"><img src="image/architecture.svg" width="600px"></div>

# Features
1. Balanced tags: 
```html 
<p>...</p>
```
2. Attributes with quoted values: `id="main"`
3. Text nodes: 
```html
<em>world</em>
```
4. Supports normal flow includes block formatting of block-level boxes, inline formatting of inline-level boxes, and relative positioning of block-level and inline-level boxes.


# Limitations
1. Does not handle the case where an inline box contains a block-level child.
2. Does not handle ignorance of wrong code.
3. Nested CSS block is not support.
4. Absolute and fixed positioning not supported.


# Reference
1. https://web.dev/articles/howbrowserswork