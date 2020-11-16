use crate::gui::components::Drawable;

type DrawableList = Vec<Box<dyn Drawable>>;

pub struct Screen {
    pub components: DrawableList,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}