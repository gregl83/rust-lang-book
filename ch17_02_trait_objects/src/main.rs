mod gui;

use crate::gui::{
    Screen,
    Button,
    SelectBox,
};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox::new(
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            )),
            Box::new(Button::new(
                width: 50,
                height: 10,
                label: String::from("OK"),
            )),
        ],
    };

    screen.run();

    println!("{:?}", screen);
}
