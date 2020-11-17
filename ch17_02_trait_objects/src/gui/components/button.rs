use crate::gui::components::Drawable;

#[derive(Debug)]
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Button {
    pub fn new(width: u32, height: u32, label: String) -> Self {
        Button {
            width,
            height,
            label,
        }
    }
}

impl Drawable for Button {
    fn draw(&self) {
        println!("Drawing Button");

        // code to actually draw a button
    }
}