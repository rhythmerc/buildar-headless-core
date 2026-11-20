//! What every buildAR headless engine hosts the same way, whatever its game. An engine's game is a library the
//! player builds from their own disc (a static recompilation run headless, stepped by the host); its repo wraps that
//! library in the engine contract ([`engine_core::EngineCore`]). The parts that don't depend on the game are here:
//!
//! - [`library`]: the game's library, loaded once and never unloaded, its C functions looked up by name.
//! - [`audio`]: the game's mix, taken from the library as it makes it.
//! - [`input`]: the controllers as a pad's axes and buttons.
//! - [`space`]: the runtime's world (metres, y up) in a game's own units and axes, and back.
//! - [`model`]: a stand-in model, for a game whose own can't be read.
//!
//! No game's names here: each game's words stay in its own repo (`tests/neutral_core.rs`).
pub mod audio;
pub mod input;
pub mod library;
pub mod model;
pub mod space;

pub use engine_core;
