use clap::Parser;
use randem::randem;

#[derive(Parser)]
#[command(name = "randem", author)]
struct RandemCLI {
    #[arg(short, long)]
    seed: Option<String>,
    #[arg(short, long)]
    include_group: Option<String>,
    #[arg(short, long)]
    exclude_group: Option<String>,
}

fn main() {
    let parsed = RandemCLI::parse();

    println!(
        "{}",
        randem(parsed.seed, parsed.include_group, parsed.exclude_group)
    );
}
