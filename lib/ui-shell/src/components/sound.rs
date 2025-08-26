use settings::Settings;

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
    if Settings::is_silent() {
        log::info!("Sound is disabled (silent mode), skipping sound playback.");
        return Ok(());
    }

    let engine =
        SoundEngine::new().map_err(|e| eyre!("Failed to initialize sound engine: {}", e))?;

    let context = SoundContext::new();
    engine.state().add_context(context.clone());

    let buffer = SoundBufferResource::new_generic(DataSource::from_memory(sound.data))
        .map_err(|_| eyre!("Failed to load sound buffer"))?;

    let source = SoundSourceBuilder::new()
        .with_buffer(buffer)
        .with_status(Status::Playing)
        .with_spatial_blend_factor(0.0)
        .build()?;

    context.state().add_source(source);
    Ok(())
}
