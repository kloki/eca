use clap::Parser;
use eca::{
    Eca,
    Init,
    RuleSet,
};
use image::{
    ImageBuffer,
    ImageError,
    Rgb,
};

mod eca;
const BLACK: image::Rgb<u8> = Rgb([0u8; 3]);
const WHITE: image::Rgb<u8> = Rgb([255u8; 3]);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Rule set to be applied
    rule_set: u8,
    /// Name of png file
    #[arg(short, long, default_value = "./output.png")]
    output_file: String,
    //width of image
    #[arg(long, default_value_t = 800)]
    width: u32,
    //height of image
    #[arg(long, default_value_t = 400)]
    height: u32,
    // how to initialize the first row
    #[arg(value_enum,short, long, default_value_t = Init::Single)]
    initalization: Init,
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    let rs = RuleSet::new(args.rule_set);
    let width = args.width as usize;
    let height = args.height as usize;
    let mut eca = Eca::new(width, args.initalization, rs);
    eca.iterate(height);

    let mut imgbuf = ImageBuffer::new(args.width, args.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if eca.iterations[y as usize][x as usize] {
            *pixel = BLACK
        } else {
            *pixel = WHITE
        }
    }

    imgbuf.save(args.output_file)?;
    Ok(())
}
