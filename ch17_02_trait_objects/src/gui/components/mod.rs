mod screen;
mod button;
mod select_box;

pub use screen::Screen;
pub use button::Button;
pub use select_box::SelectBox;

pub trait Drawable {
    fn draw(&self);
}