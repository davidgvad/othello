# Othello

A small Rust implementation of Othello/Reversi.

The project is split into two clear parts:

- `src/lib.rs` contains the reusable game engine.
- `src/main.rs` contains the playable terminal frontend.

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
- a trait-based greedy AI strategy
- recursive directional scanning for captured discs

The terminal frontend includes:

- a clean 8x8 board with row and column coordinates
- highlighted legal moves for the current player
- score and turn display
- friendly commands: `hint`, `ai`, `undo`, `score`, `help`, and `quit`
- graceful handling for invalid input and invalid moves

Rust features used in this project include enums for exact game states, structs for board coordinates and scores, pattern matching for move logic, `Result` for recoverable errors, ownership-friendly immutable queries, iterators and closures for move selection, traits for AI strategy behavior, cloning for undo history, recursion for directional board traversal, and tests for the core rules.

Run the playable terminal version with:

```bash
cargo run
```

Run the tests with:

```bash
cargo test
```
