use rusqlite::Connection;

use super::tables::{
    pokemon::{PokemonDTO, PokemonTableRepository, PokemonTableRepositoryError},
    types::{TypesDTO, TypesTableRepository, TypesTableRepositoryError},
};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Database, PokemonTableRepositoryError> {
        let conn = Connection::open("tools/sqlite3/pokedex.db")?;
        Ok(Database { conn })
    }
}

impl PokemonTableRepository for Database {
    fn fetch(&self, number: i32) -> Result<PokemonDTO, PokemonTableRepositoryError> {
        let mut stmt = self.conn.prepare("SELECT number, name FROM pokemon WHERE number = ?")?;
        let pokemon = stmt.query_row([number], |row| {
            Ok(PokemonDTO {
                number: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        Ok(pokemon)
    }

    fn fetch_all(&self) -> Result<Vec<PokemonDTO>, PokemonTableRepositoryError> {
        let mut stmt = self.conn.prepare("SELECT number, name FROM pokemon")?;
        let pokemon_iter = stmt.query_map([], |row| {
            Ok(PokemonDTO {
                number: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut pokemon = Vec::new();
        for p in pokemon_iter {
            pokemon.push(p?);
        }

        Ok(pokemon)
    }
}

impl TypesTableRepository for Database {
    fn fetch(&self, number: i32) -> Result<TypesDTO, TypesTableRepositoryError> {
        let sql_cmd = "SELECT pokemon_number, primary_type, secondary_type FROM pokemon_types WHERE number = ?";
        let mut stmt = self.conn.prepare(sql_cmd)?;
        let types = stmt.query_row([number], |row| {
            Ok(TypesDTO {
                number: row.get(0)?,
                primary_type: row.get(1)?,
                secondary_type: row.get(2)?,
            })
        })?;

        Ok(types)
    }

    fn fetch_all(&self) -> Result<Vec<TypesDTO>, TypesTableRepositoryError> {
        let sql_cmd = "SELECT pokemon_number, primary_type, secondary_type FROM pokemon_types WHERE number = ?";
        let mut stmt = self.conn.prepare(sql_cmd)?;
        let types_iter = stmt.query_map([], |row| {
            Ok(TypesDTO {
                number: row.get(0)?,
                primary_type: row.get(1)?,
                secondary_type: row.get(2)?,
            })
        })?;

        let mut types = Vec::new();
        for p in types_iter {
            types.push(p?);
        }

        Ok(types)
    }
}
