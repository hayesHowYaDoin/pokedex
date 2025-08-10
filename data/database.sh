#!/bin/bash

sqlite3 "$DATABASE_FILE" < "$./pokedex.sql"
