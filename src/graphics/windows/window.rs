// тут ми передаем все что нужно для создания окна, а именно: название окна, ширину и высоту окна

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub impl Window {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Window { title, width, height }
    }
}