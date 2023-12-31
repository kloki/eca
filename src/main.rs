use clap::Parser;
use eca::RuleSet;
use image::{
    ImageBuffer,
    ImageError,
    Rgb,
};
use rand::Rng;

mod eca;
const BLACK: image::Rgb<u8> = Rgb([0u8; 3]);
const WHITE: image::Rgb<u8> = Rgb([255u8; 3]);

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Init {
    Random,
    Single,
}

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

    let mut world = vec![vec![false; width]; height];

    match args.initalization {
        Init::Single => world[0][(width / 2) as usize] = true,
        Init::Random => {
            let mut rng = rand::thread_rng();
            for x in 0..(width - 1) {
                world[0][x] = rng.gen();
            }
        }
    }

    for y in 1..(height - 1) {
        for x in 0..(width - 1) {
            let l = {
                if x > 0 {
                    world[y - 1][x - 1]
                } else {
                    false
                }
            };
            let m = world[y - 1][x];

            let r = {
                if x < width {
                    world[y - 1][x + 1]
                } else {
                    false
                }
            };
            world[y][x] = rs.apply_rules(l, m, r);
        }
    }

    let mut imgbuf = ImageBuffer::new(args.width, args.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if world[y as usize][x as usize] {
            *pixel = BLACK
        } else {
            *pixel = WHITE
        }
    }

    imgbuf.save(args.output_file)?;
    Ok(())
}
