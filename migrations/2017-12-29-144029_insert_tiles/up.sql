INSERT INTO tiles DEFAULT VALUES;
INSERT INTO tile_roads (tile_id, road_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 2, 0);

INSERT INTO tiles DEFAULT VALUES;

INSERT INTO tiles DEFAULT VALUES;
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 0, 0);
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 1, 0);
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 2, 0);
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 3, 0);

INSERT INTO tiles DEFAULT VALUES;
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 1, 0);
INSERT INTO tile_roads (tile_id, road_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 0, 0);
INSERT INTO tile_roads (tile_id, road_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 2, 0);

INSERT INTO tiles DEFAULT VALUES;
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 0, 0);

INSERT INTO tiles DEFAULT VALUES;
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 1, 0);
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 3, 0);

INSERT INTO tiles DEFAULT VALUES;
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 0, 0);
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 2, 0);

INSERT INTO tiles DEFAULT VALUES;
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 1, 0);
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 3, 1);

INSERT INTO tiles DEFAULT VALUES;
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 1, 0);
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 2, 1);

INSERT INTO tiles DEFAULT VALUES;
INSERT INTO tile_cities (tile_id, city_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 0, 0);
INSERT INTO tile_roads (tile_id, road_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 1, 0);
INSERT INTO tile_roads (tile_id, road_side, tile_group) VALUES (currval('tiles_id_seq'::regclass), 2, 0);
