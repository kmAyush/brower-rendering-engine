type PropertyMap = HashMap<String, Value>;

struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

fn matches(element:&ElementData, selector:&Selector) -> bool {
    match selector {
        Simple(s) => matches_simple_selector(element, s);
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
fn matches_simple_selector(element: &ElementData, selector:&SimpleSelector) -> bool {
    // Check for tag
    if selector.tag_name.iter().any(|name| element.tag_name != *name) {
        return false;
    }
    // check for id
    if selector.id.iter().any(|id| element.id() != Some(id)) {
        return false;
    }
    // check for class
    if selector.class.iter().any(|class| !element.classes().contains(class.as_str())) {
        return false;
    }
    return false;
}
// BUILDING STYLE TREE

type MatchedRule<'a> = (Specificity, &'a Rule);

// If 'rule' matches 'element' return a 'MatchedRule' 
fn match_rule<'a>(elem: &ElementData, rule:&'a Rule) -> Option<MatchedRule<'a>>{
    // find highest specificity matching selector
    rule.selector.iter()
        .find(|selector| matches(element, selector))
        .map(|selector| (selector.specificity(), rule))
}
// find all css rules that match the given element
fn matching_rules<'a>(elem:&ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(element, rule)).collect()
}
// apply styles to a single element
fn specified_values(element:) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(element, stylesheet);

    rules.sort_by(| &(a,_), &(b,_) a.cmp(&b));
    for(_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    return values;
}
// apply a stylesheet to an entire DOM TREE
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node:root,
        specified_values: match root.node_type {
            Element(ref element) => specified_values(element, stylesheet),
            Text(_) => HashMap::new()
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}
