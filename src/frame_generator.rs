pub struct FrameGenerator {
    pub height: usize,
    pub width: usize,
    pub frame: Vec<String>
}

pub trait Generator {
    fn new(height:usize, width: usize) -> Self;
    fn init_generator(&mut self);
    fn get_frame(& self) ->String;
    fn create_line(&self, range_to: usize, is_first_line: bool) -> String;
}

impl Generator for FrameGenerator {
    fn new(height: usize, width: usize) -> FrameGenerator {
        FrameGenerator { height, width, frame: Vec::new()}
    }
    fn init_generator(&mut self) {
        let line_length = 20;
        let mut vector: Vec<String> = Vec::new();
        vector.push(self.create_line(line_length, true));
        for _ in 1..10 {
            let string = self.create_line(line_length, false);
            vector.push(string)
        }
        vector.push(self.create_line(line_length, true));
        self.frame = vector;
    }
    fn create_line(&self, range_to: usize, is_first_line: bool) -> String {
        let mut vector: Vec<char> = Vec::new();
        vector.push('|');
        for _ in 1..range_to{
            if is_first_line {
                vector.push('-');
            } else {
                vector.push(' ');
            }
        }
        vector.push('|');
        vector.iter().collect()
    }
    fn get_frame(&self) -> String{
        self.frame.join("\n")
    }
}

