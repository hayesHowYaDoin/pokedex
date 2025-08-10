use color_eyre::{eyre::eyre, Result};
use fyrox_sound::buffer::SoundBufferResourceExtension;
use fyrox_sound::{
    buffer::{DataSource, SoundBufferResource},
    context::SoundContext,
    engine::SoundEngine,
    source::{SoundSourceBuilder, Status},
};

use ui_core::components::Sound;

pub fn play_sound(sound: Sound) -> Result<()> {
    // Initialize sound engine with default output device.
    let engine =
        SoundEngine::new().map_err(|e| eyre!("Failed to initialize sound engine: {}", e))?;

    // Create new context.
    let context = SoundContext::new();

    // Register context in the engine.
    engine.state().add_context(context.clone());

    // Load sound buffer.
    let door_open_buffer = SoundBufferResource::new_generic(DataSource::from_memory(sound.data))
        .map_err(|_| eyre!("Failed to load sound buffer"))?;

    // Create generic source (without spatial effects) using that buffer.
    let source = SoundSourceBuilder::new()
        .with_buffer(door_open_buffer)
        .with_status(Status::Playing)
        // Ensure that no spatial effects will be applied.
        .with_spatial_blend_factor(0.0)
        .build()?;

    // Each sound sound must be added to context, context takes ownership on source
    // and returns pool handle to it by which it can be accessed later on if needed.
    context.state().add_source(source);
    Ok(())
}
