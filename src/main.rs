mod cs;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "FILES")]
    input: Vec<PathBuf>,

    #[structopt(short, long)]
    format: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
