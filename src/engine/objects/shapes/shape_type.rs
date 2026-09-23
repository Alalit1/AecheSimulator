use crate::engine::assets::resources::Resources;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
pub enum ShapeType {
    Sphere,
    Cube,
    Cylinder,
}