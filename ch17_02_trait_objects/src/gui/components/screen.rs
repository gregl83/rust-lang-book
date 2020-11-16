use crate::gui::components::Drawable;

#[derive(Debug)]
pub struct Screen<T: Drawable> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where
        T: Drawable,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}