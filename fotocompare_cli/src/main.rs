use clap::Parser;

use dircompare::compare;

#[derive(Parser, Debug)]
#[command( about = "FotoCompare CLI", long_about = None)]
struct Args {
    /// Left (source) folder
    #[arg(index = 1)]
    src: String,

    /// Right (target) folder
    #[arg(index = 2)]
    dest: String,

    /// With image orientation
    #[arg(short, long)]
    orientation: bool,
}

fn main() {
    let args = Args::parse();

    // println!("src:{}", args.src);
    // println!("dest:{}", args.dest);
    // println!("orientation:{}", args.orientation);

    compare(&args.src, &args.dest);
}
