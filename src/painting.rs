type DisplayList = Vec<DisplayCommand>;

enum DisplayCommand{
    SolidColor(Color, Rect),
}

fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    return list;
}
fn render_layout_box(list:&mut DisplayList, layout_box:&LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}
fn render_background(list:&mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(
        |color| list.push(
            DisplayCommand::SolidColor(color, 
            layout_box.dimensions.border_box())
        )
    );
}
fn get_color(layout_box: &LayoutBox, name:&str) -> Option<Color> {
    match layout_box.box_type {
        BlockNode(style) | InlineNode(style) => match style.value(name) {
            Some(Value::ColorValue(color)) => Some(color),
            _ => None
        },
        AnonymousBlock => None
    }
}
fn render_borders(list: &mut DisplayList, layout_box:&LayoutBox) {
    let color = match get_color(layout_box, "border-color"){
        Some(color) => color,
        _ => return
    };
    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    list.push(DisplayCommand::SolidColor(color, Rect{
        x:border_box.x,
        y:border_box.y,
        width: d.border.left,
        heigth: border_box.height,
    }));

    list.push(DisplayCommand::SolidColor(color, Rect{
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.right.
    }));

    list.push(DisplayCommand::SolidColor(color, Rect{
        x:border_box.x,
        y:border_box.y,
        width: border_box.width,
        height: d.border.top,
    }));

    list.push(DisplayCommand::SolidColor(color, Rect{
        x:border_box.x,
        y:border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom,
    }));
}

struct Canvas {
    pub(crate) pixels : Vec<Color>,
    pub(crate) width : usize,
    pub(crate) height : usize,
}

impl Canvas {
    fn new(width:usize, height:usize) -> Canvas {
        let white = Color { r:255, g:255, b:255, a:255};
        Canvas {
            pixels: vec![white; width*height],
            width,
            height
        }
    }
    fn paint_item(&mut self, item: &DisplayCommand) {
        match item {
            &DisplayCommand::SolidColor(color, rect) => {
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in (y0 .. y1){
                    for x in (x0..x1){
                        self.pixels[x+y*self.width] = color;
                    }
                }
            }
        }
    }
}

pub fn paint(layout_root: &LayoutBox, bounds: Rect) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(bounds.width as usize, bounds.heigth as usize);
    for item in display_list {
        canvas.paint_item(&item);
    }
    return canvas;
}