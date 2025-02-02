#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sound {
  pub data: Vec<u8>,
}

impl Sound {
    pub fn new(data: Vec<u8>) -> Self {
        Sound { data }
    }
}