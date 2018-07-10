ALTER TABLE game_players
DROP COLUMN turn_order,
DROP COLUMN score;

ALTER TABLE game_tiles
ADD COLUMN meeple_location INTEGER;
