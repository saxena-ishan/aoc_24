use clap::Parser;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day_1::solve(),
        2 => day_2::solve(),
        3 => day_3::solve(),
        4 => day_4::solve(),
        _ => eprintln!("invalid arg"),
    }
}
