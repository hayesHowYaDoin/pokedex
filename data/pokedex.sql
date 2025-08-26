.bail on
.timeout 10000

-- Create table for data/csv/pokemon_sizes.csv
DROP TABLE IF EXISTS "pokemon_sizes";
CREATE TABLE "pokemon_sizes" ("id" INT NOT NULL, "height_dm" INT NOT NULL, "weight_dg" INT NOT NULL);

-- Import data into pokemon_sizes (skip header)
.mode csv
.import --skip 1 "data/csv/pokemon_sizes.csv" "pokemon_sizes"

-- Create table for data/csv/stat_names.csv
DROP TABLE IF EXISTS "stat_names";
CREATE TABLE "stat_names" ("stat_id" INT NOT NULL, "local_language_id" INT NOT NULL, "name" TEXT NOT NULL);

-- Import data into stat_names (skip header)
.mode csv
.import --skip 1 "data/csv/stat_names.csv" "stat_names"

-- Create table for data/csv/pokemon_descriptions.csv
DROP TABLE IF EXISTS "pokemon_descriptions";
CREATE TABLE "pokemon_descriptions" ("species_id" INT NOT NULL, "version_id" INT NOT NULL, "language_id" INT NOT NULL, "flavor_text" TEXT NOT NULL);

-- Import data into pokemon_descriptions (skip header)
.mode csv
.import --skip 1 "data/csv/pokemon_descriptions.csv" "pokemon_descriptions"

-- Create table for data/csv/pokemon_types.csv
DROP TABLE IF EXISTS "pokemon_types";
CREATE TABLE "pokemon_types" ("pokemon_id" INT NOT NULL, "type_id" INT NOT NULL, "slot" INT NOT NULL);

-- Import data into pokemon_types (skip header)
.mode csv
.import --skip 1 "data/csv/pokemon_types.csv" "pokemon_types"

-- Create table for data/csv/pokemon_genders.csv
DROP TABLE IF EXISTS "pokemon_genders";
CREATE TABLE "pokemon_genders" ("id" INT NOT NULL, "gender_rate" INT NOT NULL);

-- Import data into pokemon_genders (skip header)
.mode csv
.import --skip 1 "data/csv/pokemon_genders.csv" "pokemon_genders"

-- Create table for data/csv/pokemon_stats.csv
DROP TABLE IF EXISTS "pokemon_stats";
CREATE TABLE "pokemon_stats" ("pokemon_id" INT NOT NULL, "stat_id" INT NOT NULL, "base_stat" INT NOT NULL, "effort" INT NOT NULL);

-- Import data into pokemon_stats (skip header)
.mode csv
.import --skip 1 "data/csv/pokemon_stats.csv" "pokemon_stats"

-- Create table for data/csv/pokemon.csv
DROP TABLE IF EXISTS "pokemon";
CREATE TABLE "pokemon" ("id" INT NOT NULL, "identifier" TEXT NOT NULL, "species_id" INT NOT NULL);

-- Import data into pokemon (skip header)
.mode csv
.import --skip 1 "data/csv/pokemon.csv" "pokemon"

-- Create table for data/csv/abilities.csv
DROP TABLE IF EXISTS "abilities";
CREATE TABLE "abilities" ("id" INT NOT NULL, "identifier" TEXT NOT NULL, "generation_id" INT NOT NULL, "is_main_series" INT NOT NULL);

-- Import data into abilities (skip header)
.mode csv
.import --skip 1 "data/csv/abilities.csv" "abilities"

-- Create table for data/csv/pokemon_species_names.csv
DROP TABLE IF EXISTS "pokemon_species_names";
CREATE TABLE "pokemon_species_names" ("pokemon_species_id" INT NOT NULL, "local_language_id" INT NOT NULL, "name" TEXT NOT NULL, "genus" TEXT NOT NULL);

-- Import data into pokemon_species_names (skip header)
.mode csv
.import --skip 1 "data/csv/pokemon_species_names.csv" "pokemon_species_names"

-- Create table for data/csv/versions.csv
DROP TABLE IF EXISTS "versions";
CREATE TABLE "versions" ("id" TEXT NOT NULL, "version_group_id" TEXT NOT NULL, "identifier" TEXT NOT NULL);

-- Import data into versions (skip header)
.mode csv
.import --skip 1 "data/csv/versions.csv" "versions"

-- Create table for data/csv/pokemon_abilities.csv
DROP TABLE IF EXISTS "pokemon_abilities";
CREATE TABLE "pokemon_abilities" ("pokemon_id" INT NOT NULL, "ability_id" INT NOT NULL, "is_hidden" INT NOT NULL, "slot" INT NOT NULL);

-- Import data into pokemon_abilities (skip header)
.mode csv
.import --skip 1 "data/csv/pokemon_abilities.csv" "pokemon_abilities"

-- Create table for data/csv/types.csv
DROP TABLE IF EXISTS "types";
CREATE TABLE "types" ("id" INT NOT NULL, "identifier" TEXT NOT NULL);

-- Import data into types (skip header)
.mode csv
.import --skip 1 "data/csv/types.csv" "types"
