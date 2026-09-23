use crate::engine::assets::resources::Resources;
use crate::engine::objects::transform::transform::Transform;
use crate::engine::objects::collisions::collision_shape::CollisionShape;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyData {
    pub resource: Resource,

    pub trasform: Transform,
    
    pub collision: CollisionShape,

    pub body_status_data: BodyStatusData,
 
}

impl BodyData {
    pub fn new(
        resource: Resource,
        shape_type: ShapeType,
        body_status_data: BodyStatusData,
    ) -> Self {
        Self {
            resource,
            shape_type,
            body_status_data,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
            mass: 1.0,
        }
    }
}