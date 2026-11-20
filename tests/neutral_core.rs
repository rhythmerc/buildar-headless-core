//! The core stays neutral: no game's names in it. A game's words belong in its own engine's repo.
use std::path::Path;

/// Words that name one game, its platform, or what it is played with.
const GAME_WORDS: &[&str] = &[
    "mario", "kart", "karts", "mkw", "mkwii", "kcl", "wii", "gamecube", "wipeout", "wip3out", "wo3", "ship", "ships",
    "playstation", "psx", "ps1", "skate", "skater", "thug", "thug2",
];

/// Whole words only, so "shipped" or "skateboard" don't count, but every word of an identifier does.
fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_ascii_lowercase)
}

#[test]
fn the_core_names_no_game() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut found = Vec::new();
    for file in ["Cargo.toml", "src/lib.rs", "src/library.rs", "src/audio.rs", "src/input.rs", "src/space.rs", "src/model.rs"] {
        let text = std::fs::read_to_string(root.join(file)).unwrap();
        for (line, l) in text.lines().enumerate() {
            for word in words(l) {
                if GAME_WORDS.contains(&word.as_str()) {
                    found.push(format!("{file}:{}: {word}", line + 1));
                }
            }
        }
    }
    let sources = std::fs::read_dir(root.join("src")).unwrap().count();
    assert_eq!(sources, 6, "a new source file: add it to the list above");
    assert!(found.is_empty(), "a game's words in the core (move them to its engine's repo):\n{}", found.join("\n"));
}

#[test]
fn whole_words_only() {
    assert_eq!(words("shipped skateboard, A-kart").collect::<Vec<_>>(), ["shipped", "skateboard", "a", "kart"]);
}
