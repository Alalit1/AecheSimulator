use std::fs;
use std::error::Error;
use std::path::Path;

fn save_scene(
    scene: &Resource,
    path: &Path,
) -> Result<(), Box<dyn Error>> {

    let json = serde_json::to_string_pretty(scene)?;

    fs::write(path, json)?;

    Ok(())
}
//save_scene(&scene, "mars.json")?;