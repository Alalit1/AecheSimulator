use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    id: u64,
    name: String,
}

impl Resource {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

impl Resources for Resource {
    fn id(&self) -> u64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}