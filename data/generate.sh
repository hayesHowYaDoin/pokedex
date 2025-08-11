#!/bin/bash

sqlite3 "$DATABASE_FILE" < "$PROJECT_ROOT/data/pokedex.sql"
