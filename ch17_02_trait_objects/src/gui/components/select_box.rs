use crate::gui::components::Drawable;

#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> Self {
        SelectBox {
            width,
            height,
            options,
        }
    }
}

impl Drawable for SelectBox {
    fn draw(&self) {
        println!("Drawing SelectBox");

        // code to actually draw a select box
    }
}