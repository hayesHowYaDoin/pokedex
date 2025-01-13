#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sound {
  pub source: String,
}

impl Sound {
    pub fn new() -> Self {
        Sound { source: "./test_assets/sound.wav".to_string() }
    }
}