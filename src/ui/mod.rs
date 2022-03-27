use gtk::{prelude::*, Application, ApplicationWindow};
pub fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Courier")
        .default_width(1920)
        .default_height(1080)
        .build();

    // Present window
    window.present();
}

/// Initializes the application
pub fn init_app(appid: &str) -> Application {
    // GTK4 
    let app = Application::builder()
        .application_id(appid)
        .build();
    app
}

pub fn init_courier() -> Application {
    let app = init_app("jmdaemon.github.io.courier");
    app.connect_activate(build_ui);
    app
}

pub fn run_app(app: &Application) {
    // Accept command line arguments but don't do anything
    // This is a temporary hack to be able to pass in command line arguments
    // Run the application
    app.run_with_args(&[""]);
}
