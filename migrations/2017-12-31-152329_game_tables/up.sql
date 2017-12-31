CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  current_player_id INTEGER,
  current_state TEXT,
);

CREATE TABLE players (
  id SERIAL PRIMARY KEY
);

CREATE TABLE game_players (
  id SERIAL PRIMARY KEY,
  game_id INTEGER NOT NULL REFERENCES games (id),
  player_id INTEGER NOT NULL REFERENCES players (id),
);

CREATE TABLE game_tiles (
  id SERIAL PRIMARY KEY,
  game_id INTEGER NOT NULL REFERENCES games (id),
  tile_id INTEGER NOT NULL REFERENCES tiles (id),
);