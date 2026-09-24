use crate::engine::assets::resources::Resources;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollisionShape {
    pub resource: Resource,
    pub collision_shape_data: CollisionShapeData,
}

pub impl CollisionShape {
    pub fn new(resource: Resource, collision_shape_data: CollisionShapeData) -> Self {
        Self {
            resource,
            collision_shape_data,
        }
    }
    pub get_collision_shape_data(&self) -> &CollisionShapeData {
        &self.collision_shape_data
    }
    pub set_collision_shape_data(&mut self, collision_shape_data: CollisionShapeData) {
        self.collision_shape_data = collision_shape_data;
    }
    pub get_resource(&self) -> &Resource {
        &self.resource
    }
    pub set_resource(&mut self, resource: Resource) {
        self.resource = resource;
    }
    pub 
}