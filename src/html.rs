use crate::dom;
use std::collections::HashMap;

struct Parser { // Make an index of html code snippet.
    pos:usize,
    input:String,
}

impl Parser {
    fn next_char(&self) -> char { // Peek next char
        self.input[self.pos..].chars().next().unwrap();
    }

    fn start_with(&self, s: &str) -> bool { // Is it starts with "<"
        self.input[self.pos ..].starts_with(s)
    }

    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len();
        } else {
            panic!("Expected {:?} at byte {}.", s, self.pos);
        }
    }

    fn end_of_line(&self) -> bool {
        self.pos >= self.input.len();
    }

    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        return c;
    }

    fn consume_till_true(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.end_of_line() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }
    fn consume_whitespace(&mut self){ // consume if whitespace
        self.consume_till_true(char::is_whitespace);
    }
    fn parse_name(&mut self) -> String { // consume if alphanumeric
        self.consume_till_true(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    fn classify_node(&mut self) -> dom::Node {
        // Classify node
        if self.starts_with("<") {
            self.parse_element();
        } else {
            self.parse_text();
        }
    }
    fn parse_text(&mut self) -> dom::Node { // this is paragraph </p>
        dom::text(self.consume_till_true(|c| c!='<'))
    }
    fn parse_element(&mut self) -> dom::Node { // Parse element <html>
        self.expect("<"); // opening tag
        let tag_name = self.parse_name();
        let attrs = self.parse_attr();
        self.expect(">");

        let children = self.classify_node();

        self.expect("</"); // closing tag
        self.expect(tag_name);
        self.expect(">");

        return dom::elem(tag_name, attrs, children);
    }
    fn parse_attr(&mut self) -> (String, String) { // class = "container-fluid" to (class, container-fluid)
        let name = self.parse_name();
        self.expect("=");
        let value = self.parse_quoted();
        return (name, value);
    }
    fn parse_qouted(&mut self) -> String { 
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_till_true(|c| c != open_quote);
        let close_quote = self.consume_char();
        assert_eq!(open_quote, close_quote);
        return value;
    }
    fn parse_attributes(&mut self) -> dom::AttrMap { // Map of (name, value)
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.end_of_line() || self.starts_with("</"){
                break;
            }
            nodes.push(self.classify_node());
        }
        return nodes;
    }
    pub fn parse(source : String) -> dom::Node { // Parse HTML document return DOM tree

        let mut nodes = Parser { pos:0, input:source }.parse_nodes();
        // Parser initialize(0, String) and parse_nodes() return Vec<dom::Node>
        // Example input : <html><head><title>My Title</title></head><body><h1>Hello, world!</h1></body></html>
        // dom::Node {
        //      children: Vec<Node>, 
        //      node_type: NodeType::Element(ElementData{ tag_name: "html", attrs: HashMap::new() })}
        
        if nodes.len() == 1 { 
            return nodes.remove(0);
        } else {
            return dom::elem("html".to_string(), HashMap::new(), nodes);
        }
    }
}