use crate::engine::resources::resource::Resource;
use crate::engine::sand_box::scene_data::SceneData;
use crate::engine::objects::body::Body;
use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    pub scene_data: SceneData,
    pub objects: Vec<Body>,
}

impl Scene {
    pub fn new(scene_data: SceneData) -> Self {
        Self {
            scene_data,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Body) {
        self.objects.push(object);
    }
    pub fn remove_object(&mut self, index: usize) {
        if index < self.objects.len() {
            self.objects.remove(index);
        }
    }

    // аава
    pub fn update(&mut self, delta_time: f32) {
        println!("Updating scene with delta_time: {}", delta_time);
        for object in &mut self.objects {
            object.update(delta_time);
            println!("Position: {:?}", object.body_data.transform.position);
        }
    }
}