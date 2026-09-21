// робота з ресурсами загальний блок кервування
let mut resorce_arry = []
use resorces::resource_loader::load_scene;
use resorces::resource_path::file_search;
use resorces::resource_write::write_resource_file;
use resorces::resource_saver::save_scene;
use std::path::PathBuf;

pub fn read_resource(
    index: u32,
    resorce_arry: &mut Vec<SceneData>,
) -> Result<SceneData, Box<dyn Error>> {

    let index = index as usize;

    // 1. Перевіряємо, чи ресурс уже є у списку
    if let Some(resource) = resorce_arry.get(index) {
        return Ok(resource.clone());
    }

    // 2. Якщо ресурсу немає — відкриваємо вибір файлу
    let path: PathBuf = file_search()
        .ok_or("Файл ресурсу не вибрано")?;

    // 3. Завантажуємо ресурс
    let resource = load_scene(&path)?;

    // 4. Зберігаємо ресурс у файл
    write_resource_file(&resource)?;

    // 5. Додаємо ресурс у список
    resorce_arry.push(resource.clone());

    // 6. Повертаємо його іншій частині програми
    Ok(resource)
}


pub fn save_resource(resource: &Resource,path: &Path,) -> Result<(), Box<dyn Error>> {
    save_scene(resource, path)
}

pub fn write_resource(resource: &Resource,field: &str,value: &str) -> Result<(), Box<dyn Error>> {
    resource_mod = write_resource_file(resource, field, value)?;
    save_scene(resource_mod, path)
}
