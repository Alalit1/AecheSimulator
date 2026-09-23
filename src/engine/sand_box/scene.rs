
struct Scene {
    pub scene_data: SceneData,
    pub objects: Vec<Body>,
}

impl Scene {
    pub fn new(size: [f32; 3]) -> Self {
        Scene { scene_data: SceneData::new(size), objects: Vec::new() }
    }
    pub fn add_object(&mut self, object: Body) {
        self.objects.push(object);
    }
    pub fn remove_object(&mut self, index: usize) {
        if index < self.objects.len() {
            self.objects.remove(index);
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        for object in &mut self.objects {
            object.update(delta_time);
        }
    }
}