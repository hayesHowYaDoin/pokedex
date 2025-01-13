use color_eyre::Result;
use fyrox_resource::io::FsResourceIo;
use fyrox_sound::buffer::SoundBufferResourceExtension;
use fyrox_sound::{
    buffer::{DataSource, SoundBufferResource},
    context::SoundContext,
    engine::SoundEngine,
    pool::Handle,
    source::{SoundSource, SoundSourceBuilder, Status},
};
use tokio::task::JoinHandle;

use crate::core::ui::components::Sound;

#[derive(Debug)]
pub struct SoundPlayer {
    sound: Sound,
    task: JoinHandle<()>,
}

impl SoundPlayer {
    pub fn new(sound: Sound) -> Result<Self> {
        let task = tokio::spawn(async {});

        Ok(SoundPlayer {
            sound,
            task,
        })
    }

    pub fn play(&mut self) {
        let sound = self.sound.clone();
        self.task = tokio::spawn(async move {
            play_sound(&sound);
        });
    }
}

fn play_sound(sound: &Sound) {
    // Initialize sound engine with default output device.
    let engine = SoundEngine::new().unwrap();

    // Create new context.
    let context = SoundContext::new();

    // Register context in the engine.
    engine.state().add_context(context.clone());

    // Load sound buffer.
    let door_open_buffer = SoundBufferResource::new_generic(
        fyrox_sound::futures::executor::block_on(DataSource::from_file(
            &sound.source,
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
}
