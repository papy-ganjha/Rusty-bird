pub struct FrameGenerator {
    pub height: usize,
    pub width: usize
}

pub trait Generator {
    fn new(height:usize, width: usize) -> Self;
    fn init_generator(self) -> Vec<char>;
}

impl Generator for FrameGenerator {
    fn new(height: usize, width: usize) -> FrameGenerator {
        FrameGenerator { height, width}
    }
    fn init_generator(self) -> Vec<char>{
        let vector = Vec::new();
        for i in 1..5 {
            println!("{i}");
        }
        vector
    }
}

