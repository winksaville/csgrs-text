use std::{env, fs::File, io::Read};

// Alias the library’s generic CSG type;
use csgrs::csg::CSG;
//type CSG<T> = csgrs::csg::CSG<T>;

fn main() {
    // Check for the correct number of command line arguments
    if env::args().len() != 4 {
        eprintln!("Usage: text text_scale font_file");
        std::process::exit(1);
    }

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let text = args[1].clone();
    let text_scale = args[2].parse::<f64>().unwrap();
    let font_path = std::path::Path::new(args[3].as_str());

    // Get Font data
    let mut f = File::open(font_path.to_str().unwrap()).unwrap();
    let mut font_data: Vec<u8> = Vec::new();
    let size = Read::read_to_end(&mut f, &mut font_data).unwrap();
    if size == 0 {
        eprintln!("Error reading font file");
        std::process::exit(1);
    }

    let csg_text: CSG<f64> = CSG::text(&text, &font_data, text_scale, None);
    let text_3d = csg_text.extrude(1.0).rotate(90.0, 0.0, 0.0);
    let text_3d_bb = text_3d.bounding_box();
    println!("text_3d_bb.extents()={:?}", text_3d_bb.extents());

    // Write the result as an ASCII STL:
    let font_base = font_path.file_stem().unwrap().to_str().unwrap();
    let name = &format!("text_{text}_text_scale-{text_scale:0.3}_font-{font_base}");
    let stl = text_3d.to_stl_ascii(name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
