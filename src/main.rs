extern crate azul;

use azul::{prelude::*, widgets::{button::Button}};

struct MyDataModel { }

impl Layout for MyDataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
    	let button = Button::with_label("Click Me!").dom();

        Dom::div()
        	.with_child(button)
    }
}

fn main() {

	macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/src/style.css")) }

	let css = css::override_native(include_str!(CSS_PATH!())).unwrap();
    let mut app = App::new(MyDataModel { }, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css).unwrap();
    app.run(window).unwrap();
}