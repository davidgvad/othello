# Othello

A small Rust implementation of Othello/Reversi.

The project is split into two clear parts:

- `src/lib.rs` contains the reusable game engine.
- `src/main.rs` contains the clickable desktop frontend built with `eframe`/`egui`.

The engine handles:

- standard board setup
- typed board positions
- legal move detection
- directional disc flipping
- turn switching
- skipped turns when a player has no legal moves
- game-over detection
- score counting and winner detection
- structured move errors with helpful messages
- hint and best-move scoring based on flip count
- trait-based AI strategies
- greedy local move selection for hints
- recursive minimax search for strategic AI play
- recursive directional scanning for captured discs

The desktop frontend includes:

- a clean clickable 8x8 board with row and column coordinates
- highlighted legal moves for the current player
- score, turn, and game-over display
- buttons for greedy hint, recursive AI move, undo, and restart
- graceful messages for invalid moves

Rust features used in this project include enums for exact game states, structs for board coordinates and scores, pattern matching for move logic, `Result` for recoverable errors, ownership-friendly immutable queries, iterators and closures for move selection, traits for AI strategy behavior, cloning for undo history, recursion for directional board traversal and minimax search, and tests for the core rules. The GUI demonstrates separation of concerns because the desktop frontend reuses the same backend engine instead of rewriting Othello rules.

Run the playable desktop version with:

```bash
cargo run
```

Run the tests with:

```bash
cargo test
```
