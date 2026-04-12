use adw::prelude::*;
use gtk::gio;

use crate::{config::app_id, window::Window};

mod config;
mod models;
mod scraper;
mod window;

fn main() -> gtk::glib::ExitCode {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");
    let app = adw::Application::builder().application_id(app_id()).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);

    window.present();
}
