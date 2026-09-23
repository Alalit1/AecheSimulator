use crate::engine::assets::resources::Resources;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shape {
    pub resource: Resource,
    // розміри об'єкта
    pub dimensions: [f32; 3],
}

impl Shape {
    pub fn new(
        resource: Resource,
        dimensions: [f32; 3],
    ) -> Self {
        Self {
            resource,
            shape_data,
            dimensions,
        }
    }

    pub get_dimensions(&self) -> [f32; 3] {
        self.dimensions
    }
    pub set_dimensions(&mut self, dimensions: [f32; 3]) {
        self.dimensions = dimensions;
    }
    pub get_shape_data(&self) -> &ShapeData {
        &self.shape_data
    }
    pub set_shape_data(&mut self, shape_data: ShapeData) {
        self.shape_data = shape_data;
    }
    pub get_resource(&self) -> &Resource {
        &self.resource
    }
    pub set_resource(&mut self, resource: Resource) {
        self.resource = resource;
    }
    pub get_volume(&self) -> f32 {
        self.dimensions[0] * self.dimensions[1] * self.dimensions[2]
    }
} 


pub trait Shape {
    fn volume(&self) -> f32;
    fn surface_area(&self) -> f32;
}