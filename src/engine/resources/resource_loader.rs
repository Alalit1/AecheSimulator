use std::fs;
use std::error::Error;
use crate::engine::resources::resource::Resource;
use std::path::Path;

pub fn load_scene(
    path: &Path,
) -> Result<Resource, Box<dyn Error>> {
    let json = std::fs::read_to_string(path)?;

    let scene: Resource =
        serde_json::from_str(&json)?;

    Ok(scene)
}

//let scene = load_scene("mars.json")?;

//println!("{:#?}", scene);
//let scene = load_scene(
 //   "src/engine/assets/resources/mars.json")?;


