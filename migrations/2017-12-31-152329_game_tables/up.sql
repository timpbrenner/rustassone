CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  current_player_id INTEGER,
  current_state TEXT
);

CREATE TABLE players (
  id SERIAL PRIMARY KEY,
  username TEXT
);

CREATE TABLE game_players (
  id SERIAL PRIMARY KEY,
  game_id INTEGER NOT NULL REFERENCES games (id),
  player_id INTEGER NOT NULL REFERENCES players (id)
);

CREATE TABLE game_tiles (
  id SERIAL PRIMARY KEY,
  game_id INTEGER NOT NULL REFERENCES games (id),
  tile_id INTEGER NOT NULL REFERENCES tiles (id),
  player_id INTEGER REFERENCES players (id),
  meeple_location INTEGER,
  row_offset INTEGER NOT NULL,
  column_offset INTEGER NOT NULL
);
