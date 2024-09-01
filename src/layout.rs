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
    
    fn layout_block(&mut self, containing_block:Dimensions) {
        // Child width is dependent of parent width, but parent height depends on child

        // Calculate parent element width 
        self.calculate_block_width(containing_block);

        self.calculate_block_position(containing_block);
        
        // Add children block dimension recursively to containing_block
        self.layout_block_children();
        
        self.calculate_block_height();
    }

    fn calculate_block_width(&mut self, containing_block:Dimensions) {
       let style = self.get_style_node();
        
        // initialize width with "auto"
        let auto=Keywords("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());
        
        // initialize margin, padding border = 0
        let zero = Length(0.0, Px);
        
        // set margin_left to margin-left's value otherwise margin's value 
        // otherwise zero
        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right","margin", &zero);
       	let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);
        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total = sum([&margin_left, &margin_right, &border_left, &border_right,
                    &padding_left, &padding_right, &width].iter().map(|v| v.to_px()));
        
        // if width is not auto and the total is wider than the container, treat auto margins as 0.
        if width != auto && total > containing_block.content.width {
            if margin_left == auto {margin_left = Length(0.0, Px);}
            if margin_right == auto {margin_right = Length(0.0, Px);}
        }
        let underflow = containing_block.content.width - total;
        
        match(width == auto, margin_left == auto, margin_right == auto) {
            (false, false, false) => { margin_right = Length(margin_rigth.to_px() + underflow, Px);}
            (false, false, true) => { margin_right = Length(underflow, Px); }
            (false, true, false) => {margin_left = Length(underflow, Px);}

            // if width is auto, all auto becomes 0.0 px
            (true, _, _) => {
                if margin_left == auto {margin_left=Length(0.0, Px);}
                if margin_right == auto {margin_right=Length(0.0, Px);}

                if underflow >= 0.0 {
                    width = Length(underflow, Px);
                } else {
                    width = Length(0.0, Px);
                }
            }
            (false, true, true) => {
                margin_left == Length(underflow/2.0, Px);
                margin_right == Length(underflow/2.0, Px);
            }
        }
    }
    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        let zero = Length(0.0, Px);

        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style.lookup("border-top-width", "border-width", &zero).to_px();
        d.border.bottom = style.lookup("border-bottom-width", "border-width", &zero).to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();
        
        // total width size of box
        d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;
        
        // total vertical size of box
        d.content.y = containing_block.content.height + containing_block.content.y + d.margin.top + d.border.top + d.padding.top;
    }
    fn layout_block_children(&mut self) {
        for child in &mut self.children {
            child.layout(self.dimensions);
            self.dimensions.content.height += child.dimensions.margin_box().height;
        }
    }
    fn calculate_block_height(&mut self) {
        // if explicit height provided assign it to the block
        if let Some(Length(h, Px)) = self.get_style_node().value("height"){
            self.dimensions.content.height = h;
        }
    }
}
impl Dimensions {
    fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }
    fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }
    fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)

        // Margin collapsing can be implemented
        // It allows bottom margin of box to overlap top margin of next box if needed
    }
}
impl Rect {
    fn expanded_by(self, edge: Edge_sizes) -> Rect {
        Rect {
            x       :self.x - edge.left,
            y       :self.y - edge.top,
            width   :self.width + edge.left + edge.right, 
            height  :self.height + edge.top + edge.bottom, 
        }
    }
}

