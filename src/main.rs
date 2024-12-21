use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

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

        // Show the window
        window.show();
    });

    // Run the application
    app.run();
}
