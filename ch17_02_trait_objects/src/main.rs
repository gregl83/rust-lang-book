mod gui;

use crate::gui::{
    Screen,
    Button,
    SelectBox,
};

fn main() {
    println!("Creating Screen");

    let screen = Screen {
        components: vec![
            Box::new(SelectBox::new(
                75,
                10,
                vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            )),
            Box::new(Button::new(
                50,
                10,
                String::from("OK"),
            )),
        ],
    };

    screen.run();
}
