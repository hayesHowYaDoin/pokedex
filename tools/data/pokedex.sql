CREATE TABLE IF NOT EXISTS pokemon (
    number INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (number)
);

CREATE TABLE IF NOT EXISTS types (
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (name)
);

CREATE TABLE IF NOT EXISTS pokemon_types (
    number INT NOT NULL,
    primary_type VARCHAR(255) NOT NULL,
    secondary_type VARCHAR(255),
    PRIMARY KEY (number)
);

CREATE TABLE IF NOT EXISTS pokemon_descriptions {
    number INT NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY (number)
}

CREATE TABLE IF NOT EXISTS pokemon_stats {
    number INT NOT NULL,
    hp INT NOT NULL,
    attack INT NOT NULL,
    defense INT NOT NULL,
    special_attack INT NOT NULL,
    special_defense INT NOT NULL,
    speed INT NOT NULL,
    PRIMARY KEY (number)
}

CREATE TABLE IF NOT EXISTS pokemon_attributes {
    number INT NOT NULL,
    height_feet INT NOT NULL,
    weight_pounds INT NOT NULL,
    category VARCHAR(255) NOT NULL,
    ability_1 VARCHAR(255) NOT NULL,
    ability_2 VARCHAR(255),
    male_rate INT NOT NULL,
    female_rate INT NOT NULL,
    PRIMARY KEY (number)
}

CREATE TABLE IF NOT EXISTS pokemon_abilities {
    number INT NOT NULL,
    ability_1 VARCHAR(255) NOT NULL,
    ability_2 VARCHAR(255),
    ability_hidden VARCHAR(255),
    PRIMARY KEY (number)
}

CREATE TABLE IF NOT EXISTS pokemon_images {
    number INT NOT NULL,
    front_default BLOB NOT NULL,
    PRIMARY KEY (number)
}


CREATE TABLE IF NOT EXISTS pokemon_sound {
    number INT NOT NULL,
    sound BLOB NOT NULL,
    PRIMARY KEY (number)
}

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
INSERT OR IGNORE INTO pokemon (number, name) VALUES (13, 'Weedle');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (14, 'Kakuna');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (15, 'Beedrill');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (16, 'Pidgey');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (17, 'Pidgeotto');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (18, 'Pidgeot');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (19, 'Rattata');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (20, 'Raticate');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (21, 'Spearow');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (22, 'Fearow');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (23, 'Ekans');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (24, 'Arbok');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (25, 'Pikachu');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (26, 'Raichu');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (27, 'Sandshrew');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (28, 'Sandslash');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (29, 'Nidoran♀');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (30, 'Nidorina');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (31, 'Nidoqueen');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (32, 'Nidoran♂');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (33, 'Nidorino');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (34, 'Nidoking');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (35, 'Clefairy');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (36, 'Clefable');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (37, 'Vulpix');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (38, 'Ninetales');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (39, 'Jigglypuff');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (40, 'Wigglytuff');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (41, 'Zubat');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (42, 'Golbat');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (43, 'Oddish');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (44, 'Gloom');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (45, 'Vileplume');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (46, 'Paras');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (47, 'Parasect');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (48, 'Venonat');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (49, 'Venomoth');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (50, 'Diglett');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (51, 'Dugtrio');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (52, 'Meowth');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (53, 'Persian');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (54, 'Psyduck');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (55, 'Golduck');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (56, 'Mankey');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (57, 'Primeape');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (58, 'Growlithe');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (59, 'Arcanine');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (60, 'Poliwag');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (61, 'Poliwhirl');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (62, 'Poliwrath');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (63, 'Abra');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (64, 'Kadabra');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (65, 'Alakazam');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (66, 'Machop');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (67, 'Machoke');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (68, 'Machamp');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (69, 'Bellsprout');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (70, 'Weepinbell');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (71, 'Victreebel');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (72, 'Tentacool');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (73, 'Tentacruel');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (74, 'Geodude');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (75, 'Graveler');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (76, 'Golem');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (77, 'Ponyta');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (78, 'Rapidash');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (79, 'Slowpoke');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (80, 'Slowbro');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (81, 'Magnemite');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (82, 'Magneton');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (83, 'Farfetch''d');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (84, 'Doduo');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (85, 'Dodrio');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (86, 'Seel');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (87, 'Dewgong');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (88, 'Grimer');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (89, 'Muk');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (90, 'Shellder');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (91, 'Cloyster');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (92, 'Gastly');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (93, 'Haunter');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (94, 'Gengar');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (95, 'Onix');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (96, 'Drowzee');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (97, 'Hypno');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (98, 'Krabby');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (99, 'Kingler');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (100, 'Voltorb');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (101, 'Electrode');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (102, 'Exeggcute');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (103, 'Exeggutor');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (104, 'Cubone');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (105, 'Marowak');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (106, 'Hitmonlee');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (107, 'Hitmonchan');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (108, 'Lickitung');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (109, 'Koffing');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (110, 'Weezing');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (111, 'Rhyhorn');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (112, 'Rhydon');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (113, 'Chansey');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (114, 'Tangela');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (115, 'Kangaskhan');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (116, 'Horsea');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (117, 'Seadra');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (118, 'Goldeen');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (119, 'Seaking');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (120, 'Staryu');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (121, 'Starmie');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (122, 'Mr. Mime');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (123, 'Scyther');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (124, 'Jynx');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (125, 'Electabuzz');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (126, 'Magmar');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (127, 'Pinsir');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (128, 'Tauros');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (129, 'Magikarp');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (130, 'Gyarados');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (131, 'Lapras');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (132, 'Ditto');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (133, 'Eevee');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (134, 'Vaporeon');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (135, 'Jolteon');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (136, 'Flareon');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (137, 'Porygon');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (138, 'Omanyte');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (139, 'Omastar');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (140, 'Kabuto');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (141, 'Kabutops');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (142, 'Aerodactyl');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (143, 'Snorlax');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (144, 'Articuno');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (145, 'Zapdos');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (146, 'Moltres');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (147, 'Dratini');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (148, 'Dragonair');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (149, 'Dragonite');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (150, 'Mewtwo');
INSERT OR IGNORE INTO pokemon (number, name) VALUES (151, 'Mew');

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

INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (1, 'Grass', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (2, 'Grass', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (3, 'Grass', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (4, 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (5, 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (6, 'Fire', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (7, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (8, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (9, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (10, 'Bug', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (11, 'Bug', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (12, 'Bug', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (13, 'Bug', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (14, 'Bug', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (15, 'Normal', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (16, 'Normal', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (17, 'Normal', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (18, 'Normal', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (19, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (20, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (21, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (22, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (23, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (24, 'Electric', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (25, 'Electric', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (26, 'Ground', 'Electric');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (27, 'Ground', 'Electric');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (28, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (29, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (30, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (31, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (32, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (33, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (34, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (35, 'Fairy', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (36, 'Fairy', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (37, 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (38, 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (39, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (40, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (41, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (42, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (43, 'Poison', 'Grass');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (44, 'Poison', 'Grass');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (45, 'Poison', 'Grass');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (46, 'Bug', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (47, 'Bug', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (48, 'Bug', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (49, 'Bug', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (50, 'Ground', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (51, 'Ground', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (52, 'Ground', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (53, 'Ground', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (54, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (55, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (56, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (57, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (58, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (59, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (60, 'Water', 'Psychic');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (61, 'Water', 'Psychic');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (62, 'Water', 'Psychic');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (63, 'Psychic', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (64, 'Psychic', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (65, 'Psychic', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (66, 'Fighting', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (67, 'Fighting', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (68, 'Fighting', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (69, 'Grass', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (70, 'Grass', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (71, 'Grass', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (72, 'Water', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (73, 'Water', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (74, 'Rock', 'Ground');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (75, 'Rock', 'Ground');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (76, 'Rock', 'Ground');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (77, 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (78, 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (79, 'Water', 'Psychic');
INSERT OR IGNORE INTO pokemon_types (number, primary_tTypesDTOpe, secondary_type) VALUES (87, 'Water', 'Ice');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (88, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (89, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (90, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (91, 'Water', 'Ice');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (92, 'Ghost', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (93, 'Ghost', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (94, 'Ghost', 'Poison');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (95, 'Rock', 'Ground');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (96, 'Psychic', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (97, 'Psychic', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (98, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (99, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (100, 'Electric', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (101, 'Electric', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (102, 'Grass', 'Psychic');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (103, 'Grass', 'Psychic');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (104, 'Ground', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (105, 'Ground', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (106, 'Fighting', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (107, 'Fighting', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (108, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (109, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (110, 'Poison', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (111, 'Ground', 'Rock');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (112, 'Ground', 'Rock');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (113, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (114, 'Grass', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (115, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (116, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (117, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (118, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (119, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (120, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (121, 'Water', 'Psychic');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (122, 'Psychic', 'Fairy');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (123, 'Bug', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (124, 'Ice', 'Psychic');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (125, 'Electric', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (126, 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (127, 'Bug', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (128, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (129, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (130, 'Water', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (131, 'Water', 'Ice');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (132, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (133, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (134, 'Water', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (135, 'Electric', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (136, 'Fire', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (137, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (138, 'Rock', 'Water');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (139, 'Rock', 'Water');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (140, 'Rock', 'Water');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (141, 'Rock', 'Water');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (142, 'Rock', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (143, 'Normal', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (144, 'Ice', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (145, 'Electric', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (146, 'Fire', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (147, 'Dragon', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (148, 'Dragon', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (149, 'Dragon', 'Flying');
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (150, 'Psychic', NULL);
INSERT OR IGNORE INTO pokemon_types (number, primary_type, secondary_type) VALUES (151, 'Psychic', NULL);

INSERT OR IGNORE INTO pokemon_descriptions (number, description) VALUES (1, 'For some time after its birth, it grows by gaining nourishment from the seed on its back.');

INSERT OR IGNORE INTO pokemon_stats (number, hp, attack, defense, special_attack, special_defense, speed) VALUES (1, 45, 49, 49, 65, 65, 45);

INSERT OR IGNORE INTO pokemon_attributes (number, height_feet, weight_pounds, category, ability_1, ability_2, male, female) VALUES ('2’04”', '15.2lbs', 'Seed', 'Overgrow', 'Chlorophyll', 875, 125);
