-- for users
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    elo_rating INTEGER DEFAULT 1200,
    games_played INTEGER DEFAULT 0,
    games_won INTEGER DEFAULT 0,
    games_lost INTEGER DEFAULT 0,
    games_drawn INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP
);

-- for all games
CREATE TABLE games (
    game_id SERIAL PRIMARY KEY,
    white_player_id INTEGER REFERENCES users(user_id),
    black_player_id INTEGER REFERENCES users(user_id),
    result VARCHAR(10) CHECK (result IN ('white', 'black', 'draw', 'ongoing')),
    start_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMP,
    initial_position VARCHAR(100) DEFAULT 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
    time_control VARCHAR(50),
    increment INTEGER DEFAULT 0,
    termination_reason VARCHAR(50)
);

-- individual moves of a game
CREATE TABLE moves (
    move_id SERIAL PRIMARY KEY,
    game_id INTEGER REFERENCES games(game_id),
    move_number INTEGER NOT NULL,
    move_notation VARCHAR(10) NOT NULL,
    move_uci VARCHAR(5),
    position_fen VARCHAR(100) NOT NULL,
    time_taken INTEGER, -- time in seconds
    evaluation FLOAT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(game_id, move_number)
);

-- game metadata
CREATE TABLE game_metadata (
    metadata_id SERIAL PRIMARY KEY,
    game_id INTEGER REFERENCES games(game_id),
    eco_code VARCHAR(3),
    opening_name VARCHAR(100),
    tags JSONB -- might not need this, but just in case for edge cases
);

-- Create indexes for efficient querying
CREATE INDEX idx_games_white_player ON games(white_player_id);
CREATE INDEX idx_games_black_player ON games(black_player_id);
CREATE INDEX idx_moves_game_id ON moves(game_id);
CREATE INDEX idx_metadata_game_id ON game_metadata(game_id);
CREATE INDEX idx_metadata_eco ON game_metadata(eco_code);
