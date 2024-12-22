use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CssProvider};
use gtk::gdk;

// Imports Tools
mod Tools;

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("com.Xethium.NitricRecorder")
        .build();

    // Connect to "activate" signal of the application
    app.connect_activate(|app| {
        // Create a new window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Nitric Recorder")
            .default_width(1000)
            .default_height(600)
            .build();
        window_style_css();
        window.fullscreen();
        // Show the window
        window.show();
        Tools::tool();
    });

    // Run the application
    app.run();
}

// Handle Loading Css
fn window_style_css() {
    // Get the default display
    let display = gdk::Display::default().expect("Could not get default display.");
    // Create new CSS provider
    let provider = CssProvider::new();
    // Set the style provider priority to APPLICATION level so that the application's custom CSS overrides theme styles
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // Path of the css
    let css_data = include_str!("./Style/WindowStyle.css");
    // Checks if the image exist
    if !css_data.is_empty() {
        provider.load_from_data(css_data);
        gtk::style_context_add_provider_for_display(&display, &provider, priority);
    } else {
        eprintln!("Warning: The style.css file is empty or not found.");
    }
}
