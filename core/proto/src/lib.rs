/// Placeholder for message types & framing.
#[derive(Debug)]
pub struct Frame {
    pub bytes: Vec<u8>,
}

impl Frame {
    pub fn new(b: impl Into<Vec<u8>>) -> Self { Self { bytes: b.into() } }
}

