use crate::engine::assets::resources::Resources;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RectangleShape {
    pub resource: Resource,
    pub shape: Shape,
    pub width: f32,
    pub height: f32,
}

impl Shape for RectangleShape {
    pub fn new(resource: Resource, shape: Shape, width: f32, height: f32) -> Self {
        Self {
            resource,
            shape,
            width,
            height,
        }
    }
}

impl ShapeTrait for RectangleShape {
    pub fn get_dimensions(&self) -> [f32; 3] {
        [self.width, self.height, 0.0]
    }
    pub fn set_dimensions(&mut self, dimensions: [f32; 3]) {
        self.width = dimensions[0];
        self.height = dimensions[1];
    }
    pub fn get_resource(&self) -> &Resource {
        &self.resource
    }
    pub fn set_resource(&mut self, resource: Resource) {
        self.resource = resource;
    }
    pub fn get_volume(&self) -> f32 {
        self.width * self.height
    }
}