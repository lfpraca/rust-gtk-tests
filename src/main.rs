use std::cell::Cell;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Cell::new(0);

    button.connect_clicked(move |button| {
        number.set(number.get() + 1);
        button.set_label(&format!("Increase {}", number.get()));
    });

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Plugin Configuration Manager")
        // .child(&button)
        .build();

    window.set_child(Some(&button));

    // Present window
    window.present();
}