use ch15_02_deref::MyBox;

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let my_y = MyBox::new(x);

    assert_eq!(5, *my_y);
}
