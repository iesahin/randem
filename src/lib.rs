use emoji::{self, Emoji};
use rand::seq::IteratorRandom;
use rand::SeedableRng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn randem(
    seed: Option<String>,
    include_group: Option<String>,
    exclude_group: Option<String>,
) -> String {
    let mut rng = if let Some(seed) = seed {
        let hashed = hash_string(&seed);
        rand::rngs::StdRng::seed_from_u64(hashed)
    } else {
        rand::rngs::StdRng::from_entropy()
    };

    let include_group = include_group.map(|s| s.to_lowercase());
    let exclude_group = exclude_group.map(|s| s.to_lowercase());

    let emoji_filter = |e: &Emoji| {
        if let Some(include_group) = &include_group {
            e.group.to_lowercase().contains(include_group)
        } else if let Some(exclude_group) = &exclude_group {
            !e.group.to_lowercase().contains(exclude_group)
        } else {
            true
        }
    };

    let all_emoji: Vec<&Emoji> = emoji::lookup_by_name::iter_emoji().collect();
    all_emoji
        .into_iter()
        .filter(|e| emoji_filter(e))
        .choose(&mut rng)
        .map(|e| e.glyph.to_string())
        .unwrap_or_else(|| "".to_string())
}
