-- Table: players
CREATE TABLE players (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL,
    address TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: rooms
CREATE TABLE rooms (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    owner_id INTEGER NOT NULL REFERENCES players(id),
    name TEXT NOT NULL,
    password TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: room_players (Join Table for many-to-many relationship)
CREATE TABLE room_players (
    player_id INTEGER NOT NULL REFERENCES players(id),
    room_id INTEGER NOT NULL REFERENCES rooms(id),
    joined_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (player_id, room_id)
);

-- Add indices for potential performance improvements (optional but recommended)
CREATE INDEX idx_players_address ON players (address);
CREATE INDEX idx_rooms_owner_id ON rooms (owner_id);
CREATE INDEX idx_room_players_player_id ON room_players (player_id);
CREATE INDEX idx_room_players_room_id ON room_players (room_id);
