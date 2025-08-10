#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod mapper {}
pub use mapper::*;
mod file {
    use std::path::PathBuf;
    use std::sync::LazyLock;
    pub static DATABASE_PATH: LazyLock<PathBuf> = LazyLock::new(|| match std::env::var(
            "POKEDEX_DATABASE_PATH",
        )
        .ok()
    {
        Some(p) => PathBuf::from(p.trim_end_matches('/')),
        None => PathBuf::from("/usr/share/rich_pokedex/pokedex.db"),
    });
    pub static ASSETS_PATH: LazyLock<PathBuf> = LazyLock::new(|| match std::env::var(
            "POKEDEX_ASSETS_PATH",
        )
        .ok()
    {
        Some(p) => PathBuf::from(p.trim_end_matches('/')),
        None => PathBuf::from("/usr/share/rich_pokedex/assets"),
    });
}
mod tables {
    use derive_crud::{Read, ReadAll};
    #[crud_table("pokemon")]
    pub struct PokemonDTO {
        #[crud_id]
        pub id: i64,
        pub species_id: i64,
        pub identifier: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PokemonDTO {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "PokemonDTO",
                "id",
                &self.id,
                "species_id",
                &self.species_id,
                "identifier",
                &&self.identifier,
            )
        }
    }
    impl PokemonDTO {
        /// Reads an entry from the database by its ID.
        ///
        /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
        /// The field annotated with `#[crud_id]` is used as the identifier for the table.
        pub async fn read(id: i64) -> anyhow::Result<Self> {
            let database_url = std::env::var("DATABASE_URL")?;
            let pool = sqlx::SqlitePool::connect(&database_url).await?;
            let item = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let arg0 = &(id);
                        let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::<
                            '_,
                        >::default();
                        query_args
                            .reserve(
                                1usize,
                                0
                                    + ::sqlx::encode::Encode::<
                                        sqlx::sqlite::Sqlite,
                                    >::size_hint(arg0),
                            );
                        let query_args = ::core::result::Result::<
                            _,
                            ::sqlx::error::BoxDynError,
                        >::Ok(query_args)
                            .and_then(move |mut query_args| {
                                query_args.add(arg0).map(move |()| query_args)
                            });
                        ::sqlx::__query_with_result::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >("SELECT * FROM \"pokemon\" WHERE id = ?", query_args)
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                #[allow(non_snake_case)]
                                let sqlx_query_as_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(0usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_identifier = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(1usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_species_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(2usize)?
                                    .into();
                                ::std::result::Result::Ok(PokemonDTO {
                                    id: sqlx_query_as_id,
                                    identifier: sqlx_query_as_identifier,
                                    species_id: sqlx_query_as_species_id,
                                })
                            })
                    }
                }
            }
                .fetch_one(&pool)
                .await?;
            Ok(item)
        }
    }
    impl PokemonDTO {
        /// Reads all entries from the database.
        ///
        /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
        pub async fn read_all() -> anyhow::Result<Vec<Self>> {
            let database_url = std::env::var("DATABASE_URL")?;
            let pool = sqlx::SqlitePool::connect(&database_url).await?;
            let items: Vec<PokemonDTO> = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let query_args = ::core::result::Result::<
                            _,
                            ::sqlx::error::BoxDynError,
                        >::Ok(
                            <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::<
                                '_,
                            >::default(),
                        );
                        ::sqlx::__query_with_result::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >("SELECT * FROM \"pokemon\"", query_args)
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                #[allow(non_snake_case)]
                                let sqlx_query_as_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(0usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_identifier = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(1usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_species_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(2usize)?
                                    .into();
                                ::std::result::Result::Ok(PokemonDTO {
                                    id: sqlx_query_as_id,
                                    identifier: sqlx_query_as_identifier,
                                    species_id: sqlx_query_as_species_id,
                                })
                            })
                    }
                }
            }
                .fetch_all(&pool)
                .await?;
            Ok(items)
        }
    }
    #[crud_table("types")]
    pub struct TypeDTO {
        #[crud_id]
        pub id: i64,
        pub identifier: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TypeDTO {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "TypeDTO",
                "id",
                &self.id,
                "identifier",
                &&self.identifier,
            )
        }
    }
    impl TypeDTO {
        /// Reads an entry from the database by its ID.
        ///
        /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
        /// The field annotated with `#[crud_id]` is used as the identifier for the table.
        pub async fn read(id: i64) -> anyhow::Result<Self> {
            let database_url = std::env::var("DATABASE_URL")?;
            let pool = sqlx::SqlitePool::connect(&database_url).await?;
            let item = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let arg0 = &(id);
                        let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::<
                            '_,
                        >::default();
                        query_args
                            .reserve(
                                1usize,
                                0
                                    + ::sqlx::encode::Encode::<
                                        sqlx::sqlite::Sqlite,
                                    >::size_hint(arg0),
                            );
                        let query_args = ::core::result::Result::<
                            _,
                            ::sqlx::error::BoxDynError,
                        >::Ok(query_args)
                            .and_then(move |mut query_args| {
                                query_args.add(arg0).map(move |()| query_args)
                            });
                        ::sqlx::__query_with_result::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >("SELECT * FROM \"types\" WHERE id = ?", query_args)
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                #[allow(non_snake_case)]
                                let sqlx_query_as_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(0usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_identifier = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(1usize)?
                                    .into();
                                ::std::result::Result::Ok(TypeDTO {
                                    id: sqlx_query_as_id,
                                    identifier: sqlx_query_as_identifier,
                                })
                            })
                    }
                }
            }
                .fetch_one(&pool)
                .await?;
            Ok(item)
        }
    }
    impl TypeDTO {
        /// Reads all entries from the database.
        ///
        /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
        pub async fn read_all() -> anyhow::Result<Vec<Self>> {
            let database_url = std::env::var("DATABASE_URL")?;
            let pool = sqlx::SqlitePool::connect(&database_url).await?;
            let items: Vec<TypeDTO> = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let query_args = ::core::result::Result::<
                            _,
                            ::sqlx::error::BoxDynError,
                        >::Ok(
                            <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::<
                                '_,
                            >::default(),
                        );
                        ::sqlx::__query_with_result::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >("SELECT * FROM \"types\"", query_args)
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                #[allow(non_snake_case)]
                                let sqlx_query_as_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(0usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_identifier = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(1usize)?
                                    .into();
                                ::std::result::Result::Ok(TypeDTO {
                                    id: sqlx_query_as_id,
                                    identifier: sqlx_query_as_identifier,
                                })
                            })
                    }
                }
            }
                .fetch_all(&pool)
                .await?;
            Ok(items)
        }
    }
    #[crud_table("pokemon_types")]
    pub struct PokemonTypeDTO {
        #[crud_id]
        pub pokemon_id: i64,
        pub type_id: i64,
        pub slot: u32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PokemonTypeDTO {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "PokemonTypeDTO",
                "pokemon_id",
                &self.pokemon_id,
                "type_id",
                &self.type_id,
                "slot",
                &&self.slot,
            )
        }
    }
    impl PokemonTypeDTO {
        /// Reads an entry from the database by its ID.
        ///
        /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
        /// The field annotated with `#[crud_id]` is used as the identifier for the table.
        pub async fn read(id: i64) -> anyhow::Result<Self> {
            let database_url = std::env::var("DATABASE_URL")?;
            let pool = sqlx::SqlitePool::connect(&database_url).await?;
            let item = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let arg0 = &(id);
                        let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::<
                            '_,
                        >::default();
                        query_args
                            .reserve(
                                1usize,
                                0
                                    + ::sqlx::encode::Encode::<
                                        sqlx::sqlite::Sqlite,
                                    >::size_hint(arg0),
                            );
                        let query_args = ::core::result::Result::<
                            _,
                            ::sqlx::error::BoxDynError,
                        >::Ok(query_args)
                            .and_then(move |mut query_args| {
                                query_args.add(arg0).map(move |()| query_args)
                            });
                        ::sqlx::__query_with_result::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >(
                                "SELECT * FROM \"pokemon_types\" WHERE pokemon_id = ?",
                                query_args,
                            )
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                #[allow(non_snake_case)]
                                let sqlx_query_as_pokemon_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(0usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_type_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(1usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_slot = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(2usize)?
                                    .into();
                                ::std::result::Result::Ok(PokemonTypeDTO {
                                    pokemon_id: sqlx_query_as_pokemon_id,
                                    type_id: sqlx_query_as_type_id,
                                    slot: sqlx_query_as_slot,
                                })
                            })
                    }
                }
            }
                .fetch_one(&pool)
                .await?;
            Ok(item)
        }
    }
    impl PokemonTypeDTO {
        /// Reads all entries from the database.
        ///
        /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
        pub async fn read_all() -> anyhow::Result<Vec<Self>> {
            let database_url = std::env::var("DATABASE_URL")?;
            let pool = sqlx::SqlitePool::connect(&database_url).await?;
            let items: Vec<PokemonTypeDTO> = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let query_args = ::core::result::Result::<
                            _,
                            ::sqlx::error::BoxDynError,
                        >::Ok(
                            <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::<
                                '_,
                            >::default(),
                        );
                        ::sqlx::__query_with_result::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >("SELECT * FROM \"pokemon_types\"", query_args)
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                #[allow(non_snake_case)]
                                let sqlx_query_as_pokemon_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(0usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_type_id = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(1usize)?
                                    .into();
                                #[allow(non_snake_case)]
                                let sqlx_query_as_slot = row
                                    .try_get_unchecked::<::std::option::Option<i64>, _>(2usize)?
                                    .into();
                                ::std::result::Result::Ok(PokemonTypeDTO {
                                    pokemon_id: sqlx_query_as_pokemon_id,
                                    type_id: sqlx_query_as_type_id,
                                    slot: sqlx_query_as_slot,
                                })
                            })
                    }
                }
            }
                .fetch_all(&pool)
                .await?;
            Ok(items)
        }
    }
}
