CREATE TABLE meeple (
  id SERIAL PRIMARY KEY,
  tile_id INTEGER NOT NULL REFERENCES tiles (id),
  game_id INTEGER NOT NULL REFERENCES games (id),
  player_id INTEGER NOT NULL REFERENCES players (id),
  side INTEGER NOT NULL
);
