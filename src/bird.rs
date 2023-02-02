const WIDTH: u32 = 520;
const HEIGHT: u32 = 520;
const BOX_SIZE: i16 = 28;


pub struct Bird {
    x: i16,
    y: i16,
    velocity_x: i16,
    velocity_y: i16,
    gravity: i16
}


impl Bird {
     pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            velocity_x: 2,
            velocity_y: 0,
            gravity: 2,
        }
    }
    pub fn jump(&mut self) {
        self.velocity_y = 4
    }
    fn fall(&mut self) {
        self.velocity_y = self.velocity_y - self.gravity
    }

    pub fn update(&mut self) {
        if self.x <= 0 || self.x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.y <= 0 || self.y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.x += self.velocity_x;
        self.y += self.velocity_y;

        self.fall();
    }
    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.x
                && x < self.x + BOX_SIZE
                && y >= self.y
                && y < self.y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

}
