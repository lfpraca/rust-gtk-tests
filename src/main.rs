use gtk::glib::clone;
use gtk::{glib, Application, ApplicationWindow, Entry, Frame, Label, Orientation, PasswordEntry};
use gtk::{prelude::*, Button};

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
    let label_usuario = Label::builder()
        .label("Usuário:")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let entry_usuario = Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .tooltip_text("E-mail do usuário")
        .build();

    let box_usuario = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    box_usuario.append(&label_usuario);
    box_usuario.append(&entry_usuario);

    let label_senha = Label::builder()
        .label("Senha:")
        .margin_bottom(12)
        .margin_start(20)
        .margin_end(12)
        .build();

    let entry_senha = PasswordEntry::builder()
        .show_peek_icon(true)
        .margin_bottom(12)
        .margin_end(12)
        .build();

    let box_senha = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .visible(false)
        .build();
    box_senha.append(&label_senha);
    box_senha.append(&entry_senha);

    let ui_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    ui_box.append(&box_usuario);
    ui_box.append(&box_senha);

    entry_usuario.connect_changed(clone!(@weak box_senha => move |_|{
        box_senha.set_visible(true);
        println!("DEBUG: usuário alterado");
    }));

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Configurações Plugin")
        .child(&ui_box)
        .build();

    // window.set_child(Some(&button));

    // Present window
    window.present();
}
