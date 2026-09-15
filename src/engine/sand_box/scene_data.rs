use crate::engine::assets::resources::Resources;

pub struct SceneData {
    pub id: u64,
    pub name: String,

    pub size: [f32; 3],
    pub environment: EnvironmentData,
}

impl SceneData {
    pub fn new(
        id: u64,
        name: String,
        size: [f32; 3],
    ) -> Self {
        Self {
            id,
            name,
            size,
            environment: EnvironmentData::new(),
        }
    }
}

impl Resources for SceneData {
    fn id(&self) -> u64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}