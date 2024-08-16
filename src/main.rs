use clap::Parser;
use emoji::{self, Emoji};
use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Parser)]
#[command(name = "randem", author)]
struct RandemCLI {
    #[arg(short, long)]
    seed: Option<String>,
}

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let parsed = RandemCLI::parse();

    let mut rng = if let Some(seed) = parsed.seed {
        let hashed = hash_string(&seed);
        rand::rngs::StdRng::seed_from_u64(hashed)
    } else {
        rand::rngs::StdRng::from_entropy()
    };

    let all_emoji: Vec<&Emoji> = emoji::lookup_by_name::iter_emoji().collect();
    let random_emoji = all_emoji.into_iter().choose(&mut rng).unwrap();
    println!("{}", random_emoji.glyph);
}
