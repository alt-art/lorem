extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate glib;

use gtk::prelude::*;
use glib::clone;

use gtk::{ApplicationWindow, Builder, Label, Button, Adjustment, Clipboard, ComboBoxText};

mod lorem;
use lorem::Lorem;

fn generate(label: gtk::Label, min: u32, max: u32, output_type: &str) {
    let lorem = Lorem::new();
    label.set_text(&lorem.get_phrase(min, max, output_type));
}

fn build_ui(application: &gtk::Application) {

    let glade_src = include_str!("./resources/lorem.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("window").expect("Cound't get ApplicationWindow");
    let label: Label = builder.object("label").expect("Cound't get Label");
    let min_adjustment: Adjustment = builder.object("min_adjust").expect("Cound't get Adjustment");
    let max_adjustment: Adjustment = builder.object("max_adjust").expect("Cound't get Adjustment");
    let output_type: ComboBoxText = builder.object("output_type").expect("Cound't get ComboBoxText");
    let gen_button: Button = builder.object("gen").expect("Cound't get Button");
    gen_button.connect_clicked(clone!(@weak label => move |_| {
        generate(
            label,
            min_adjustment.value() as u32,
            max_adjustment.value() as u32,
            output_type.active_text().unwrap().as_str()
        );
    }));
    let copy_button: Button = builder.object("copy").expect("Cound't get Button");
    copy_button.connect_clicked(clone!(@weak label => move |_| {
        let clipboard = Clipboard::get(&gdk::SELECTION_CLIPBOARD);
        clipboard.set_text(&label.text());
    }));
    generate(label, 2, 3, "Text");
    window.set_application(Some(application));
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.altart.lorem"),
        Default::default(),
    );
    application.connect_activate(|app| {
        build_ui(app)
    });

    application.run();
}
