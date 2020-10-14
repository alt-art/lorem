extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate glib;

use gtk::prelude::*;
use gio::prelude::*;
use glib::clone;

use gtk::{ApplicationWindow, Builder, Label, Button, Adjustment, Clipboard};

use std::env::args;

mod lorem;
use lorem::Lorem;

fn generate(label: gtk::Label, min: u32, max: u32) {
    let lorem = Lorem::new();
    label.set_text(&lorem.get_phrase(min, max));
}

fn build_ui(application: &gtk::Application) {

    let glade_src = include_str!("./resources/lorem.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window").expect("Cound't get ApplicationWindow");
    let label: Label = builder.get_object("label").expect("Cound't get Label");
    let min_adjustment: Adjustment = builder.get_object("min_adjust").expect("Cound't get Adjustment");
    let max_adjustment: Adjustment = builder.get_object("max_adjust").expect("Cound't get Adjustment");
    let gen_button: Button = builder.get_object("gen").expect("Cound't get Button");
    gen_button.connect_clicked(clone!(@weak label => move |_| {
        generate(label, min_adjustment.get_value() as u32, max_adjustment.get_value() as u32);
    }));
    let copy_button: Button = builder.get_object("copy").expect("Cound't get Button");
    copy_button.connect_clicked(clone!(@weak label => move |_| {
        let clipboard = Clipboard::get(&gdk::SELECTION_CLIPBOARD);
        clipboard.set_text(&label.get_text());
    }));
    generate(label, 2, 3);
    window.set_application(Some(application));
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.altart.lorem"),
        Default::default(),
    ).expect("Initialization failed");
    application.connect_activate(|app| {
        build_ui(app)
    });

    application.run(&args().collect::<Vec<_>>());
}
