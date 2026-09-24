use std::collections::HashMap;
use crate::engine::resources::resource::Resource;

pub struct ResourceRegistry {
    resources: HashMap<u32, Resource>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: u32, resource: Resource) {
        self.resources.insert(id, resource);
    }

    pub fn get(&self, id: u32) -> Option<&Resource> {
        self.resources.get(&id)
    }
}