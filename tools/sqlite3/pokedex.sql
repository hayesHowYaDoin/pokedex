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
