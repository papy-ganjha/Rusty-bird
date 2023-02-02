pub struct FrameGenerator {
    height: usize,
    width: usize
}

impl FrameGenerator {
    pub fn new(height: u32) -> FrameGenerator {
        FrameGenerator{
            height,
            width: 50
        }
    }
}
