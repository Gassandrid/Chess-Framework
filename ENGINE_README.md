# Rust Chess Engine

A high-performance chess engine written in Rust with advanced search and evaluation features.

## Features

### Core Engine Components

1. **Bitboard Representation**
   - Efficient 64-bit bitboard representation for fast move generation
   - Optimized bit manipulation operations
   - Support for all standard chess pieces and positions

2. **Move Generation**
   - Complete pseudo-legal and legal move generation
   - Support for all special moves: castling, en passant, promotions
   - Check and checkmate detection
   - Stalemate and draw detection

3. **Position Evaluation**
   - Material counting with standard piece values
   - Piece-square tables for positional evaluation
   - Pawn structure analysis (doubled, isolated, passed pawns)
   - Mobility evaluation
   - King safety evaluation (pawn shield, open files)
   - Bishop pair bonus
   - Endgame vs middlegame evaluation

4. **Search Algorithm**
   - **Negamax with Alpha-Beta Pruning**: Efficient minimax search
   - **Iterative Deepening**: Progressive depth increase for better time management
   - **Aspiration Windows**: Narrow search windows for faster convergence
   - **Principal Variation Search (PVS)**: Optimized search for expected best moves
   - **Null Move Pruning**: Reduce search tree size safely
   - **Late Move Reductions (LMR)**: Reduce search depth for unlikely moves
   - **Quiescence Search**: Tactical stability at leaf nodes

5. **Transposition Table**
   - Zobrist hashing for position identification
   - Multi-threaded safe read/write with RwLock
   - Replacement scheme based on depth and age
   - Configurable size (default 64MB)

6. **Move Ordering**
   - Transposition table move first
   - MVV-LVA (Most Valuable Victim - Least Valuable Attacker) for captures
   - Killer move heuristic
   - History heuristic
   - Proper ordering crucial for alpha-beta efficiency

7. **UCI Protocol**
   - Full UCI (Universal Chess Interface) support
   - Compatible with chess GUIs like Arena, Cutechess, etc.
   - Commands: position, go, stop, isready, etc.

8. **Opening Book**
   - Support for custom opening books
   - Weight-based move selection
   - Can be loaded from PGN files (stub implementation)

## Engine Strength Features

The engine implements many techniques used by strong chess engines:

- **Search Depth**: Configurable depth up to 64 ply
- **Tactical Vision**: Quiescence search prevents horizon effects
- **Positional Understanding**: Comprehensive evaluation function
- **Efficiency**: Bitboard representation and transposition tables
- **Time Management**: Adaptive time allocation per move

## API Endpoints

The engine is integrated with a REST API:

- `POST /api/engine/best-move` - Get best move for a position
- `POST /api/engine/analyze` - Analyze a position
- `POST /api/engine/evaluate` - Get static evaluation
- `POST /api/engine/perft` - Run perft tests

## Command-Line Tools

### UCI Engine
```bash
cargo run --bin uci
```
Run the engine in UCI mode. Compatible with chess GUIs.

### Perft Testing
```bash
# Run test suite
cargo run --bin perft test

# Run perft on starting position
cargo run --bin perft 5

# Run perft on custom FEN
cargo run --bin perft 4 "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
```

### Engine Tournament
```bash
cargo run --bin tournament
```
Run automated tournaments to compare different engine versions.

## Testing and Evaluation System

### Elo Rating System
The engine includes an Elo rating system for tracking engine strength:
- Initial rating: 1500
- K-factor: 32 (adjustable)
- Automatic rating updates after each game

### Automated Tournaments
- Round-robin tournament support
- Alternating colors for fairness
- Configurable games per match
- Maximum move limits to prevent infinite games
- Detailed statistics and standings

### Match Statistics
For each match, the system tracks:
- Wins, losses, draws
- Win percentage
- Score (with draws counting as 0.5)
- Elo rating changes

## Usage Examples

### Testing Engine Improvements

When you make changes to the evaluation function or search algorithm:

1. **Create a new engine configuration**:
```rust
let baseline = EngineConfig::new("Baseline v1.0".to_string(), 5);
let improved = EngineConfig::new("Improved v2.0".to_string(), 5);
```

2. **Run a tournament**:
```rust
let mut tournament = Tournament::new(50, 200); // 50 games, 200 max moves
tournament.add_engine(baseline, 1500.0);
tournament.add_engine(improved, 1500.0);
tournament.run_round_robin();
```

3. **Analyze results**:
The tournament will print detailed statistics showing which version performs better.

### Integration with Frontend

The engine is designed to work with a web frontend through the REST API:

```javascript
// Get best move
const response = await fetch('/api/engine/best-move', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    depth: 10,
    time_ms: 5000
  })
});
const data = await response.json();
console.log("Best move:", data.best_move);
```

## Performance

### Perft Results (Starting Position)
- Depth 1: 20 nodes
- Depth 2: 400 nodes
- Depth 3: 8,902 nodes
- Depth 4: 197,281 nodes
- Depth 5: ~4.8M nodes
- Depth 6: ~119M nodes

### Search Performance
- **Nodes per second**: 500K - 2M NPS (depending on position)
- **Transposition table hits**: 30-50% in typical middlegame positions
- **Effective branching factor**: ~3-4 with good move ordering

## Evaluation Function Weights

### Material Values (centipawns)
- Pawn: 100
- Knight: 320
- Bishop: 330
- Rook: 500
- Queen: 900
- King: 20000

### Positional Bonuses
- Passed pawn: 10-70 cp (based on advancement)
- Doubled pawn penalty: -20 cp per extra pawn
- Isolated pawn penalty: -15 cp
- Bishop pair bonus: +50 cp
- Pawn shield: +15 cp per pawn
- Open file near king penalty: -25 cp
- Mobility: +2 cp per legal move

## Future Improvements

Potential areas for strengthening the engine:

1. **Magic Bitboards**: Faster sliding piece move generation
2. **Null Move Pruning Improvements**: Adaptive reduction depth
3. **Late Move Reductions Tuning**: Better reduction conditions
4. **Evaluation Tuning**: Use machine learning to optimize weights
5. **Multi-threading**: Parallel search (Lazy SMP)
6. **Endgame Tablebases**: Perfect play in simple endgames
7. **SEE (Static Exchange Evaluation)**: Better capture evaluation
8. **Extended Futility Pruning**: Additional pruning techniques
9. **Contempt Factor**: Adjust evaluation based on opponent strength
10. **Opening Book Expansion**: Larger, more diverse opening repertoire

## Building and Testing

```bash
# Build all binaries
cargo build --release

# Run API server
cargo run --release --bin api

# Run UCI engine
cargo run --release --bin uci

# Run perft tests
cargo run --release --bin perft test

# Run tournament
cargo run --release --bin tournament

# Run unit tests
cargo test
```

## Architecture

```
backend/src/
├── engine/
│   ├── bitboard.rs          # Bitboard representation
│   ├── position.rs          # Position and game state
│   ├── movegen.rs           # Move generation
│   ├── evaluation.rs        # Position evaluation
│   ├── search.rs            # Search algorithm
│   ├── transposition.rs     # Transposition table
│   ├── time_manager.rs      # Time management
│   ├── uci.rs              # UCI protocol
│   ├── opening_book.rs     # Opening book
│   ├── perft.rs            # Perft testing
│   ├── testing.rs          # Tournament system
│   └── mod.rs              # Module exports
├── api/
│   ├── handlers.rs         # Chess API handlers
│   ├── engine_handlers.rs  # Engine API handlers
│   ├── models.rs           # API models
│   └── mod.rs
├── chess/                  # Legacy chess implementation
└── main.rs                # API server entry point
```

## License

MIT

## Contributing

When contributing improvements to the engine:

1. Run perft tests to ensure move generation is correct
2. Test changes in tournaments against the baseline
3. Document any evaluation function changes
4. Update this README with new features

## Credits

Developed as part of the Chess Framework for Engines and Data Science project.
