# HTML and CSS Rendering Engine
<p><strong>Is it necessary to have multiple languages code for rendering a webpage on a browser, can one single language be used to build a webpage without transpiling to HTML?</strong></p><p>
A toy implementation of a browser rendering engine to understand the rendering of HTML and CSS code as well as to learn about Rust data structure by practical implementation of HashMaps, Nodes, and Trees.</p>

# Architecture

<img src="diagram/pipeline.svg"/>

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

# Storing as tree
For example test.html
```html
<html>
  <head>
    <title>Test</title>
  </head>
  <div class="outer">
    <p class="inner">
      Hello, <span id="name">world!</span>
    </p>
    <p class="inner" id="bye">
      Goodbye!
    </p>
  </div>
</html>
```
## Internal Tree node structure
HTML
```
Node (Element: html)
├── Node (Element: head)
│   └── Node (Element: title)
│       └── Node (Text: "Test")
└── Node (Element: div, class: "outer")
    ├── Node (Element: p, class: "inner")
    │   ├── Node (Text: "Hello, ")
    │   └── Node (Element: span, id: "name")
    │       └── Node (Text: "world!")
    └── Node (Element: p, class: "inner", id: "bye")
        └── Node (Text: "Goodbye!\n")
```
<hr>

Similarly for test.css,<br>
CSS
```
Stylesheet
├── Rule
│   ├── Selector: Simple (tag_name: None, id: None, class: [])
│   └── Declarations
│       └── Declaration: display = Keyword("block")
├── Rule
│   ├── Selector: Simple (tag_name: Some("span"), id: None, class: [])
│   └── Declarations
│       └── Declaration: display = Keyword("inline")
├── Rule
│   ├── Selector: Simple (tag_name: Some("html"), id: None, class: [])
│   └── Declarations
│       ├── Declaration: width = Length(600.0, Px)
│       ├── Declaration: padding = Length(10.0, Px)
│       ├── Declaration: border-width = Length(1.0, Px)
│       ├── Declaration: margin = Keyword("auto")
│       └── Declaration: background = ColorValue(Color { r: 255, g: 255, b: 255, a: 255 })
├── Rule
│   ├── Selector: Simple (tag_name: Some("head"), id: None, class: [])
│   └── Declarations
│       └── Declaration: display = Keyword("none")
├── Rule
│   ├── Selector: Simple (tag_name: None, id: None, class: ["outer"])
│   └── Declarations
│       ├── Declaration: background = ColorValue(Color { r: 0, g: 204, b: 255, a: 255 })
│       ├── Declaration: border-color = ColorValue(Color { r: 102, g: 102, b: 102, a: 255 })
│       ├── Declaration: border-width = Length(2.0, Px)
│       ├── Declaration: margin = Length(50.0, Px)
│       └── Declaration: padding = Length(50.0, Px)
├── Rule
│   ├── Selector: Simple (tag_name: None, id: None, class: ["inner"])
│   └── Declarations
│       ├── Declaration: border-color = ColorValue(Color { r: 204, g: 0, b: 0, a: 255 })
│       ├── Declaration: border-width = Length(4.0, Px)
│       ├── Declaration: height = Length(100.0, Px)
│       ├── Declaration: margin-bottom = Length(20.0, Px)
│       └── Declaration: width = Length(500.0, Px)
├── Rule
│   ├── Selector: Simple (tag_name: None, id: None, class: ["inner"])
│   ├── Selector: Simple (tag_name: None, id: Some("bye"), class: [])
│   └── Declarations
│       └── Declaration: background = ColorValue(Color { r: 255, g: 255, b: 0, a: 255 })
└── Rule
    ├── Selector: Simple (tag_name: Some("span"), id: None, class: [])
    ├── Selector: Simple (tag_name: None, id: Some("name"), class: [])
    └── Declarations
        ├── Declaration: background = Keyword("red")
        └── Declaration: color = Keyword("white")

```
```
StyledNode (Element: html)
├── Specified Values:
│   ├── display = Keyword("block")
│   ├── border-width = Length(1.0, Px)
│   ├── background = ColorValue(255, 255, 255, 255)
│   ├── width = Length(600.0, Px)
│   ├── margin = Keyword("auto")
│   └── padding = Length(10.0, Px)
└── Children:
    ├── StyledNode (Element: head)
    │   ├── Specified Values:
    │   │   └── display = Keyword("none")
    │   └── Children:
    │       └── StyledNode (Element: title)
    │           ├── Specified Values:
    │           │   └── display = Keyword("block")
    │           └── Children:
    │               └── StyledNode (Text: "Test")
    └── StyledNode (Element: div, class: "outer")
        ├── Specified Values:
        │   ├── border-color = ColorValue(102, 102, 102, 255)
        │   ├── padding = Length(50.0, Px)
        │   ├── margin = Length(50.0, Px)
        │   ├── border-width = Length(2.0, Px)
        │   ├── display = Keyword("block")
        │   └── background = ColorValue(0, 204, 255, 255)
        └── Children:
            ├── StyledNode (Element: p, class: "inner")
            │   ├── Specified Values:
            │   │   ├── height = Length(100.0, Px)
            │   │   ├── width = Length(500.0, Px)
            │   │   ├── border-color = ColorValue(204, 0, 0, 255)
            │   │   ├── border-width = Length(4.0, Px)
            │   │   ├── display = Keyword("block")
            │   │   ├── margin-bottom = Length(20.0, Px)
            │   │   └── background = ColorValue(255, 255, 0, 255)
            │   └── Children:
            │       ├── StyledNode (Text: "Hello, ")
            │       └── StyledNode (Element: span, id: "name")
            │           │   ├── display = Keyword("inline")
            │           ├── Specified Values:
            │           │   ├── background = Keyword("red")
            │           │   └── color = Keyword("white")
            │           └── Children:
            │               └── StyledNode (Text: "world!")
            └── StyledNode (Element: p, class: "inner", id: "bye")
                ├── Specified Values:
                │   ├── margin-bottom = Length(20.0, Px)
                │   ├── width = Length(500.0, Px)
                │   ├── background = ColorValue(255, 255, 0, 255)
                │   ├── display = Keyword("block")
                │   ├── border-width = Length(4.0, Px)
                │   ├── border-color = ColorValue(204, 0, 0, 255)
                │   └── height = Length(100.0, Px)
                └── Children:
                    └── StyledNode (Text: "Goodbye!\n    ")

```

# Limitations
1. Does not handle the case where an inline box contains a block-level child.
2. Does not handle ignorance of wrong code.
3. Nested CSS block is not supported.
4. Absolute and fixed positioning are not supported.


# Reference
1. Article on building a browser rendering engine, https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html
2. Article on how a browser works, https://web.dev/articles/howbrowserswork
