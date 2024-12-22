use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CssProvider};
use gtk::gdk;

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
        //window_style_css();
        window.fullscreen();
        window.set_opacity(0.3);
        // Show the window
        window.show();
    });

    // Run the application
    app.run();
}

fn window_style_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // Load the CSS from src/WindowStyle.css
    provider.load_from_data(include_str!("./Style/WindowStyle.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}
