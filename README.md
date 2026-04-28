# Othello

A small Rust implementation of Othello/Reversi.

David's core game engine is in `src/lib.rs`. It handles:

- board setup
- legal move detection
- directional disc flipping
- turn switching
- skipped turns when a player has no legal moves
- game-over detection
- score counting

Run the playable terminal version with:

```bash
cargo run
```

Run the tests with:

```bash
cargo test
```
