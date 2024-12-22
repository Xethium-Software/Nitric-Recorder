use gtk::{ApplicationWindow, Button, Fixed, Picture, FileChooserAction, FileChooserDialog, Image, Align};
use gtk::prelude::*;
use std::path::Path;
use gtk::gdk;
use ffmpeg_next as ffmpeg;
use std::process::Command;

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

    bScreenshot.connect_clicked(|_| {
        take_screenshot();
        println!("Screenshot!");
    });

    // Show everything
    tool.set_child(Some(&layout));
    tool.show();
}


pub fn take_screenshot() {
    let ffmpeg_available = Command::new("which")
        .arg("ffmpeg")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !ffmpeg_available {
        eprintln!("FFmpeg is not installed or not available in PATH.");
        return;
    }

    let output = Command::new("ffmpeg")
        .args(&[
            "-f", "x11grab",            // Grab video from the screen
            "-video_size", "1920x1080", // Screen resolution
            "-i", ":0.0",               // X11 display input (for Linux)
            "-frames:v", "1",           // Capture only 1 frame (screenshot)
            "screenshot.png",           // Output file name
        ])
        .output();

    match output {
        Ok(result) if result.status.success() => {
            println!("Screenshot saved as screenshot.png");
        }
        Ok(result) => {
            eprintln!(
                "FFmpeg command failed. STDERR: {}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
        Err(e) => {
            eprintln!("Failed to execute FFmpeg: {}", e);
        }
    }
}
