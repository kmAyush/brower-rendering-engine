extern crate getopts;
extern crate image;

use std::default::Default;
use std::io::{Read, BufWriter};
use std::fs::File;

pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod style;
pub mod painting;
pub mod pdf;

fn main() {
    let mut arg = getopts::Options::new();
    arg.optopt("h","html","HTML document", "FILENAME");
    arg.optopt("c","css","CSS stylesheet", "FILENAME");
    arg.optopt("o", "output", "Output file", "FILENAME");
    arg.optopt("f", "format", "Output file format", "png | pdf");

    let matches = arg.parse(std::env::args().skip(1)).unwrap();
    let str_arg = | flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };
    
    let is_png = match &str_arg("f", "png")[..]{
        "png" => true,
        "pdf" => false,
        x => panic!("Unknown output format: {}", x),
    };

    let html = read_source(str_arg("h", "examples/1.html"));
    let css = read_source(str_arg("c", "examples/1.css"));

    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    // Parsing and rendering
    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let output_file = str_arg("o", if png {"output.png"} else {"output.pdf"});
    let mut file = BufWriter::new(File::create(&output_file).unwrap());
    

    let ok = if is_png {
        let canvas = painting::paint(&layout_root, viewport.content);
        let (w, h) = (canvas.width as u32, canvas.height as u32);
        let img = image::ImageBuffer::from_fn(w, h, move |x,y|{
            let color = canvas.pixels[(y*w +x) as usize];
            image::Pixel::from_channels(color.r, color.g, color.b, color.a)
        });
        image::ImageRgba8(img).save(&mut file, image::PNG).is_ok();
    } else {
        pdf::render(&layout_root, viewport.content, &mut file).is_ok()
    };

    if ok {
        println!("Saved output as {}", filename)
    } else {
        println!("Error saving output as {}", filename)
    }
}

fn read_source(filename:String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}