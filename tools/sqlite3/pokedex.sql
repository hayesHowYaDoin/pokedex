CREATE TABLE IF NOT EXISTS pokemon (
    number INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (number)
);

INSERT OR IGNORE INTO pokemon (number, name) VALUES (1, 'Bulbasaur');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (2, 'Ivysaur');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (3, 'Venusaur');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (4, 'Charmander');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (5, 'Charmeleon');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (6, 'Charizard');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (7, 'Squirtle');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (8, 'Wartortle');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (9, 'Blastoise');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (10, 'Caterpie');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (11, 'Metapod');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (12, 'Butterfree');

CREATE TABLE IF NOT EXISTS types (
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (name)
    SECONDARY KEY (name)
);

INSERT OR IGNORE INTO types (name) VALUES ('Normal');
INSERT OR IGNORE INTO types (name) VALUES ('Fire');
INSERT OR IGNORE INTO types (name) VALUES ('Water');
INSERT OR IGNORE INTO types (name) VALUES ('Electric');
INSERT OR IGNORE INTO types (name) VALUES ('Grass');
INSERT OR IGNORE INTO types (name) VALUES ('Ice');
INSERT OR IGNORE INTO types (name) VALUES ('Fighting');
INSERT OR IGNORE INTO types (name) VALUES ('Poison');
INSERT OR IGNORE INTO types (name) VALUES ('Ground');
INSERT OR IGNORE INTO types (name) VALUES ('Flying');
INSERT OR IGNORE INTO types (name) VALUES ('Psychic');
INSERT OR IGNORE INTO types (name) VALUES ('Bug');
INSERT OR IGNORE INTO types (name) VALUES ('Rock');
INSERT OR IGNORE INTO types (name) VALUES ('Ghost');
INSERT OR IGNORE INTO types (name) VALUES ('Dragon');
INSERT OR IGNORE INTO types (name) VALUES ('Dark');
INSERT OR IGNORE INTO types (name) VALUES ('Steel');
INSERT OR IGNORE INTO types (name) VALUES ('Fairy');

CREATE TABLE IF NOT EXISTS pokemon_types (
    pokemon_number INT NOT NULL,
    primary_type VARCHAR(255) NOT NULL,
    secondary_type VARCHAR(255),
    PRIMARY KEY (pokemon_number),
);

INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Bulbasaur', 'Grass', 'Poison');
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Ivysaur', 'Grass', 'Poison');
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Venusaur', 'Grass', 'Poison');
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Charmander', 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Charmeleon', 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Charizard', 'Fire', 'Flying');
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Squirtle', 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Wartortle', 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Blastoise', 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Caterpie', 'Bug', NULL);
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Metapod', 'Bug', NULL);
INSERT OR IGNORE INTO pokemon_types (pokemon_number, primary_type, secondary_type) VALUES ('Butterfree', 'Bug', 'Flying');
