use clap::Parser;
use eca::{Eca, Init, RuleSet};

mod eca;
mod writer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Rule set to be applied
    rule_set: u8,
    /// Name of output file
    #[arg(short, long)]
    output_file: Option<String>,
    //width of initalization
    #[arg(long, default_value_t = 800)]
    width: u32,
    //number of iterations
    #[arg(long, default_value_t = 400)]
    iterations: u32,
    // how to initialize the first row
    #[arg(value_enum,short, long, default_value_t = Init::Single)]
    initalization: Init,
}

impl Args {
    fn get_output_file(&self) -> String {
        match &self.output_file {
            Some(s) => s.to_string(),
            None => format!("{}.png", self.rule_set),
        }
    }
}

fn main() {
    let args = Args::parse();
    let rs = RuleSet::new(args.rule_set);
    let width = args.width as usize;
    let iterations = args.iterations as usize;
    let file_name = args.get_output_file();
    let mut eca = Eca::new(width, args.initalization, rs);
    eca.iterate(iterations);
    if file_name.ends_with(".png") {
        writer::to_png(&file_name, eca.iterations).expect("failed to write image");
    } else {
        writer::to_txt(&file_name, eca.iterations).expect("failed to write to txt file")
    }
}
