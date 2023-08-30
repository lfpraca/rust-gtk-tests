use std::cell::Cell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Label, Orientation};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number_label = Label::builder().label("Number: 0").build();

    let number = Rc::new(Cell::new(0));

    // button.connect_clicked(move |button| {
    //     number.set(number.get() + 1);
    //     button.set_label(&format!("Increase {}", number.get()));
    // });

    button_increase.connect_clicked(clone!(@weak number, @weak number_label => move |_| {
    number.set(number.get() + 1);
    number_label.set_label(&format!("Number: {}", number.get()));
    println!("DEBUG: number increased");
    }));

    button_decrease.connect_clicked(clone!(@weak number_label => move |_| {
    number.set(number.get() - 1);
    number_label.set_label(&format!("Number: {}", number.get()));
    println!("DEBUG: number decreased");
    }));

    let my_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    my_box.append(&number_label);
    my_box.append(&button_increase);
    my_box.append(&button_decrease);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Plugin Configuration Manager")
        .child(&my_box)
        .build();

    // window.set_child(Some(&button));

    // Present window
    window.present();
}
