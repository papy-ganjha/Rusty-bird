struct Bird {
    x: i32,
    y: i32,
    velocity: Vec<i32>
}

trait BirdTrait {
    fn new() -> Self;
    fn fly(&self);
}

impl BirdTrait for Bird {
    fn new() -> Bird {
        let mut init_velocity = Vec::new();
        init_velocity.push(0);
        init_velocity.push(0);
        Bird{x: 0, y:0, velocity:Vec::new()}
    }
    fn fly(&self) {
        println!("x is: {} y is: {} velocity is: {}", self.x, self.y, self.velocity[0])
    }
}
