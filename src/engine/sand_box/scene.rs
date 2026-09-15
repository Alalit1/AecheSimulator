
struct Scene {
    pub scene_data: SceneData,
}

impl Scene {
    pub fn new(size: [f32; 3]) -> Self {
        Scene { scene_data: SceneData::new(size) }
    }
}