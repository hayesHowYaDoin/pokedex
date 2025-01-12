
#![allow(dead_code)]

mod shell;
mod core;

use color_eyre::eyre::Result;

use crate::shell::sql::DatabaseMapper;
use crate::shell::ratatui::app::App;


use fyrox_resource::io::FsResourceIo;
use fyrox_sound::buffer::SoundBufferResourceExtension;
use fyrox_sound::{
    buffer::{DataSource, SoundBufferResource},
    context::SoundContext,
    engine::SoundEngine,
    pool::Handle,
    source::{SoundSource, SoundSourceBuilder, Status},
};
use std::{thread, time::Duration};


async fn tokio_main() -> Result<()> {
    
    // Initialize sound engine with default output device.
    let engine = SoundEngine::new().unwrap();

    // Create new context.
    let context = SoundContext::new();

    // Register context in the engine.
    engine.state().add_context(context.clone());

    // Load sound buffer.
    let door_open_buffer = SoundBufferResource::new_generic(
        fyrox_sound::futures::executor::block_on(DataSource::from_file(
            "test_assets/sound.wav",
            // Load from the default resource io (File system)
            &FsResourceIo,
        ))
        .unwrap(),
    )
    .unwrap();

    // Create generic source (without spatial effects) using that buffer.
    let source = SoundSourceBuilder::new()
        .with_buffer(door_open_buffer)
        .with_status(Status::Playing)
        // Ensure that no spatial effects will be applied.
        .with_spatial_blend_factor(0.0)
        .build()
        .unwrap();

    // Each sound sound must be added to context, context takes ownership on source
    // and returns pool handle to it by which it can be accessed later on if needed.
    let _source_handle: Handle<SoundSource> = context.state().add_source(source);

    // Wait until sound will play completely.
    thread::sleep(Duration::from_secs(1));


    let db = DatabaseMapper::new("tools/sqlite3/pokedex.db")
        .expect("Failed to create database connection");

    let mut app = App::new(&db).expect("Failed to create application");
    app.run().await?;
  
    Ok(())
  }

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } 
    else {
        Ok(())
    }
}