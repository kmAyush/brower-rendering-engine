use std::collections::{HashMap, HashSet};

// DOM tree
pub(crate) struct Node {
    children:Vec<Node>,
    node_type: NodeType,
}

// Node can be either text or element.
enum NodeType {
    Text(String),
    Element(ElementData),
}

// Element is consist of tagname and attrs, <title>Attribute
struct ElementData {
    tag_name: String,
    attrs:AttrMap,
}

// Stores Attribute map like class="grid mt-2"
type AttrMap = HashMap<String, String>;

// Store to Node when element is text
pub(crate) fn text(data:String) -> Node {
    Node {children: Vec::new(), node_type: NodeType::Text(data)}
}

// Store to Node when element is element
// Example input : tag = div, attrs = class:container-fluid, child nodes 
pub fn element(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children, 
        node_type: NodeType :: Element(ElementData{ tag_name, attrs })
    }
}

impl ElementData{
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }
    pub fn classes(&self) -> HashSet<&str> { // Class element can contain multiple class
        match self.attributes.get("class"){
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}