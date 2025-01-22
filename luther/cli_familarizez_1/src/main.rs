use cli_familarizez_1::create_fruit_salad;
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Paet",
    about ="Bs",
)]

struct Opts{
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let num = opts.number;

    println!("{:?}", create_fruit_salad(num));
}
