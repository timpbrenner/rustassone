CREATE TABLE tiles (
  id SERIAL PRIMARY KEY
);

CREATE TABLE tile_cities (
  id SERIAL PRIMARY KEY,
  tile_id INTEGER NOT NULL REFERENCES tiles (id),
  city_side INTEGER NOT NULL,
  tile_group INTEGER NOT NULL
);

CREATE TABLE tile_roads (
  id SERIAL PRIMARY KEY,
  tile_id INTEGER NOT NULL REFERENCES tiles (id),
  road_side INTEGER NOT NULL,
  tile_group INTEGER NOT NULL
);
