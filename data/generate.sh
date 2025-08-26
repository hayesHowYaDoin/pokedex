#!/bin/bash

sqlite3 "$POKEDEX_DATABASE_PATH" < "$PROJECT_ROOT/data/pokedex.sql"
