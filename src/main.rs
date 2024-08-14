use color_eyre::eyre::Result;
use std::io::prelude::*;
use std::fs::File;
use resvg::{usvg, tiny_skia};
use clap::Parser;

/// Convert an SVG to a PNG of a given size
#[derive(Parser)]
struct Cli {
    /// The path to the SVG icon source file
    path: std::path::PathBuf,
    /// The output width
    width: u32,
    /// The output height
    height: u32,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();

    let size = tiny_skia::IntSize::from_wh(args.width, args.height).expect("Invalid size");

    let mut f = File::open(args.path)?;
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;

    let tree = usvg::Tree::from_data(data.as_slice(), &usvg::Options::default())?;
    
    let mut img = tiny_skia::Pixmap::new(size.width(), size.height()).expect("Couldn't allocate pixmap");

    let fsize = size.to_size();
    let tsize = tree.size();
    let transform = tiny_skia::Transform::from_scale(
        fsize.width() / tsize.width(),
        fsize.height() / tsize.height(),
    );

    resvg::render(&tree, transform, &mut img.as_mut());
    img.save_png("/home/tom/Pictures/test.png")?;
    Ok(())
}
