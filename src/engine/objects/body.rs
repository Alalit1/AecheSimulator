struct Body {
    pub body_data: BodyData,
}

impl Body {
    pub fn new(position: [f32; 3], velocity: [f32; 4], body_data: BodyData) -> Self {
        Body {
            position,
            velocity,
            body_data,
        }
    }

}

trait Updatable {
    fn update(&mut self, delta_time: f32);
}