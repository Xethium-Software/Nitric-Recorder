use gtk::{ApplicationWindow};
use gtk::prelude::WidgetExt;

pub fn tool() {
    let tool = ApplicationWindow::builder()
        .title("Tools")
        .default_width(600)
        .default_height(100)
        .resizable(false)
        .build();

    
    tool.show();
}