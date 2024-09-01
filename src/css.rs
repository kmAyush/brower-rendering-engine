struct Stylesheet { // List of rules
    rules: Vec<Rule>,
}

struct Rule { // list of selectors { list of declaration; }
    selectors : Vec<Selector>,
    declarations : Vec<Declaration>,
}

enum Selector {  // Add scalability to target Complex selector later on.
    Simple(SimpleSelector)
}

struct SimpleSelector { // selectors - #id, .class, body
    tag_name : Option<String>,
    id : Option<String>,
    class : Vec<String>,
}

struct Declaration { // margin-top : 5px
    name : String,
    value : Value,
}

enum Value {Keyword(String), Length(f32, Unit), ColorValue(Color),}

enum Unit {Px,}

struct Color { //rgba()
    r:u8, g:u8, b:u8, a:u8,
}

fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector {tag_name: None, id: None, class: Vec::new()};
    while !self.end_of_line() {
        match self.next_char() {
            '#' => {
                self.consume_char();
                selector.id = Some(self.parse_identifier());
            }, 
            '.' => {
                self.consume_char();
                selector.class.push(self.parse_identifier());
            }
            '*' => {
                self.consume_char();
            }
            c if valid_identifier_char(c) => {
                selector.tag_name = Some(self.parse_identifier);
            }
            _ => break
        }
    }
    return selector;
}

fn valid_identifier_char(c:char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let n_id = simple.id.iter().count();
        let n_class = simple.class.len();
        let n_tag = simple.tag_name.iter().count();
        return (n_id, n_class, n_tag);
    }
}

fn parse_rule(&mut self) -> Rule {
    Rule {
        selectors : self.parse_selectors(),
        declaration : self.parse_declarations()
    }
}

fn parse_selectors(&mut self) -> Vec<Selector> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(Selector::Simple(self.parse_simple_selector()));
        self.consume_whitespace();
        match self.next_char() {
            ',' => {self.consume_char(), self.consume_whitespace();}
            '{' => break,
            c => panic("Unexpected character {} in selector list", c)
        }
    }
    // Return selectors with specificity first, for use in matching.
    selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
    return selectors;
}