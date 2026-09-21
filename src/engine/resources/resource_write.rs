// чітання перепис 
// принімае \ файл.рес
// виполняет зним махінації 
// зберігае (ідправляе в save) \ new_файл.рес
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::error::Error;


pub fn write_resource_file(
    resource: &Resource,
    field: &str,
    value: &str,
) -> Result<Resource, Box<dyn Error>> {

    // Передаємо ресурс і параметри зміни
    let modified_resource =
        manipulate_resource_data(resource, field, value)?;

    // Тут, якщо потрібно, зберігаємо Resource у файл
    // save_resource_file(...)?;

    // Повертаємо вже змінений ресурс
    Ok(modified_resource)
}


fn manipulate_resource_data(
    resource: &Resource,
    field: &str,
    value: &str,
) -> Result<Resource, Box<dyn Error>> {


    let mut data = serde_json::to_value(resource)?;

    data[field] = value;

    let modified_resource: Resource =
        serde_json::from_value(data)?;

    Ok(modified_resource)
}
