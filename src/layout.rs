struct Dimensions {
    content:Rect,

    padding:Edge_sizes,
    border:Edge_sizes,
    margin:Edge_sizes,
}

struct Rect {
    x:f32,
    y:f32,
    width:f32,
    height:f32,
}

struct Edge_sizes {
    left:f32,
    right:f32,
    top:f32,
    bottom:f32,
}
struct Layout<'a> {
    dimensions: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}
enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock,
}
enum Display {
    Inline,
    Block,
    None,
}
impl StyledNode {
    // Return the specified_values of a property if it exists
    fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).map(|v| v.clone())
    }

    // Return the value of the 'display' property 
    fn display(&self) -> Display {
        match self.value("display") {
            Some(Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline
            },
            _ => Display::Inline
        }
    }
}
// Build Layout Tree from StyledNode 
fn build_layout_tree<'a>(style_node: &'a StyleNode<'a>) -> LayoutBox<'a> {
    // Create the root box
    let mut root = LayoutBox::new(match style_node.display(){
        Block => BlockNode(style_node),
        Inline => InlineNode(style_node),
        DisplayNone => panic!("Root node has display:none.")
    });
    // Create the descendant boxes
    for child in &style_node.children {
        match child.display() {
            Block => root.children.push(build_layout_tree(child)),
            Inline => root.get_inline_container().children.push(build_layout_tree(child)),
            DisplayNone => {}
        }
    }
    return root;
}

impl LayoutBox {
    // Constructor Function
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(), // Initialize by 0.0
            children: Vec::new(),
        }
    }
    // Add a new inline child to LayoutBox
    fn get_inline_container(&mut self) -> &mut LayoutBox {
        match self.box_type {
            InlineNode(_) | AnonymousBlock => self,
            BlockNode(_) => {
                // If we have just generated an anonymous block box,
                // keep using it.
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: AnonymousBlock, ..
                    }) => {}
                }
                self.children.last_mut().unwrap()
            }
        }
    }
    
}
