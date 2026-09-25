use serde::{Serialize,Deserialize};
use crate::engine::resources::resource::Resource;
use crate::engine::objects::transforms::transform::Transform;
//use crate::engine::objects::collisions::collision_shape::CollisionShape;


// ресур який містить даін наданн обекту форми та властивостей
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyData {
    pub resource: Resource,
    //статйс  
    pub transform: Transform,
    pub types: ObjactType,
    //pub collision: CollisionShape,

    //pub body_status_data: BodyStatusData,
    pub physics_model: PhysicsModel,
 
}

impl BodyData {
    pub fn new(
        resource: Resource,
        transform: Transform,
        //collision: CollisionShape,
        //body_status_data: BodyStatusData,
    ) -> Self {
        Self {
            resource,
            transform,
            //collision,
            //body_status_data,
            
        }
    }
}