# buildar-headless-core

What every [buildAR](https://github.com/rhythmerc/skate3-quest-ar) headless engine hosts the same way, whatever its
game. An engine's game is a library the player builds from their own disc (a static recompilation run headless,
stepped by the host); its repo wraps that library in the engine contract
([buildar-engine-core](https://github.com/rhythmerc/buildar-engine-core)'s `EngineCore`). The parts that don't depend
on the game are here:

- `library`: the game's library, loaded once and never unloaded, its C functions looked up by name.
- `audio`: the game's mix, taken from the library as it makes it.
- `input`: the controllers as a pad's axes and buttons.
- `space`: the runtime's world (metres, y up) in a game's own units and axes, and back.
- `model`: a stand-in model, for a game whose own can't be read.

No game's data, code or names are here (`tests/neutral_core.rs` checks the names). Used by
[wip3out-headless](https://github.com/rhythmerc/wip3out-headless).

Licence: GPL-3.0-or-later.
