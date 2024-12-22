use gtk::{ApplicationWindow, Button, Fixed, Picture, FileChooserAction, FileChooserDialog, Image, Align};
use gtk::prelude::*;
use std::path::Path;

pub fn tool() {
    // Create application window
    let tool = ApplicationWindow::builder()
        .title("Tools")
        .default_width(600)
        .default_height(100)
        .resizable(false)
        .build();
        
    // Create layout
    let layout = Fixed::new();
    
    // Create Button
    let bScreenshot = Button::new();
    let bScreenRecord = Button::new();
    let bGUI = Button::new();
    
    // Path for the images
    let iCamera = Image::from_file("resources/Images/camera.png");
    let iVideoCamera = Image::from_file("resources/Images/video-camera.png");
    let iGUI = Image::from_file("resources/Images/home.png");

    // Load the button
    layout.put(&bScreenshot, 100.0, 20.0);
    bScreenshot.set_size_request(100, 60);
    bScreenshot.set_child(Some(&iCamera));

    layout.put(&bScreenRecord, 250.0, 20.0);
    bScreenRecord.set_size_request(100, 60);
    bScreenRecord.set_child(Some(&iVideoCamera));

    layout.put(&bGUI, 400.0, 20.0);
    bGUI.set_size_request(100, 60);
    bGUI.set_child(Some(&iGUI));

    // Show everything
    tool.set_child(Some(&layout));
    tool.show();
}