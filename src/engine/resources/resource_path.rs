use std::path::Path;
use std::error::Error;
use rfd::FileDialog
use std::env;


pub fn file_search() -> Result<PathBuf, Box<dyn Error>> {
    let project_dir = env::current_dir()
        .expect("Не вдалося отримати current directory");

    let src_dir = project_dir.join("src");

    println!("Відкриваємо: {}", src_dir.display());

    if let Some(path) = FileDialog::new()
        .set_directory(&src_dir)
        .add_filter("JSON", &["json"])
        .pick_file()
    {
        println!("Вибраний файл: {}", path.display());
    }
}

pub fn search_folder() -> Result<PathBuf, Box<dyn Error>> {
    let project_dir = env::current_dir()
        .expect("Не вдалося отримати current directory");

    let src_dir = project_dir.join("src");

    println!("Відкриваємо: {}", src_dir.display());

    if let Some(path) = FileDialog::new()
        .set_directory(&src_dir)
        .add_filter("JSON", &["json"])
        .pick_folder()
    {
        println!("Вибраний каталог: {}", path.display());
    }
}

pub fn parser(mut name: String) -> Result<(), Box<dyn std::error::Error>>{
    // Вкажіть шлях до вашої папки
    let path = "D:\\Rust Program\\ArcheSimulator\\src\\engine\\constants"; 
    name = name + ".res";

    // Читаємо вміст папки
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();

        // Перевіряємо, чи це файл, а не підпапка
        if file_path.is_file() {
            // Виводимо ім'я файлу в консоль
            if let Some(file_name) = file_path.file_name() {
                if file_name == name.as_str() {
                    println!("Found file: {:?}", file_name);
                    // Тут ви можете додати код для обробки знайденого файлу
                }
                else {
                    println!("File not found: {:?}", name);
                }
               
            }
        }
    }

    Ok(())
}